// ----------------------------------------------------------------------------
//  confirm.rs — Confirmation module
// ----------------------------------------------------------------------------
//  Confirmation module — verifies findings with additional probes to reduce
//  false positives.
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & interface design
//  HsecDevelopers         — 测试 / テスト / testing & QA (integration, validation)
//  projectk 2091         — HyperSecurityOffensiveLabs lineage
// ----------------------------------------------------------------------------
//
//
// ---------------------------------------------------------------------------
//   LICENSE / ライセンス — GNU General Public License v3 (GPL-3.0)
// ---------------------------------------------------------------------------
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  OXIDE v8.7.2-community-edition — HyperSecurityOffensiveLabs
// ---------------------------------------------------------------------------
//
//
//  confirm.rs — 確認モジュール
//  Confirmation module — secondary probes to reduce false positives
//  発見結果に対し追加検証を行い偽陽性を排除

use crate::detection::analyzer::Finding;

//  Confirm — 偽陽性除去エンジン
//  False positive reduction engine
//  発見結果に対して追加検証プローブを実行し真の脆弱性のみを報告
pub struct Confirm;

//  確認ロジックの実装
//  Confirmation logic implementation
impl Confirm {
    //  confirm_vulnerability — 発見結果の種類に応じて確認メソッドを振り分け
    //  Routes to specific confirmation method based on finding type
    //  SQLi / LFI / XSS / CMDi / AdminPanel の各確認メソッドを呼ぶ
    pub fn confirm_vulnerability(finding: &Finding) -> bool {
        // Informational findings (parameter discovery, etc.) are always kept
        if matches!(finding.severity, crate::detection::analyzer::Severity::Info) {
            return true;
        }
        if finding.title.contains("SQLi") || finding.title.contains("SQL Injection") {
            Self::confirm_sql_injection(finding)
        } else if finding.title.contains("LFI") || finding.title.contains("File Inclusion") {
            Self::confirm_lfi(finding)
        } else if finding.title.contains("XSS") || finding.title.contains("Cross-Site") {
            Self::confirm_xss(finding)
        } else if finding.title.contains("CMDi") || finding.title.contains("Command Injection") {
            Self::confirm_cmdi(finding)
        } else if finding.title.contains("Admin") || finding.title.contains("Panel") {
            Self::confirm_admin_panel(finding)
        } else {
            matches!(finding.severity, crate::detection::analyzer::Severity::High | crate::detection::analyzer::Severity::Critical)
        }
    }

    //  confirm_sql_injection — SQLインジェクションの確認
    //  Confirms SQL injection — requires actual SQL error context
    //  実際のSQLエラーコンテキストを要求 (ORA-XXXXX, SQLSTATE 等)
    //  Cloudflare/WAFブロックページを除外  偽陽性防止
    fn confirm_sql_injection(finding: &Finding) -> bool {
        let evidence = finding.evidence.to_lowercase();
        // Require actual SQL error context, not just generic keywords
        let has_sql_error = evidence.contains("sql syntax") ||
            evidence.contains("unclosed quotation") ||
            (evidence.contains("ora-") && evidence.split("ora-").skip(1).any(|s| s.chars().next().map_or(false, |c| c.is_ascii_digit()))) ||
            evidence.contains("sqlstate") ||
            evidence.contains("incorrect syntax near") ||
            evidence.contains("column count") ||
            (evidence.contains("mysql_fetch") || evidence.contains("mysql_error") || evidence.contains("supplied argument is not a valid mysql"));
        has_sql_error &&
        !evidence.contains("cf-ray") &&
        !evidence.contains("cloudflare") &&
        !evidence.to_lowercase().contains("waf-block") && !evidence.to_lowercase().contains("waf-denied") &&
        !evidence.contains("blocked")
    }

    //  confirm_xss — XSSの確認
    //  Confirms XSS — requires explicit payload reflection
    //  <script>alert( / <img src=x onerror=alert( 等の明示的ペイロード一致が必要
    //  HTMLエンコードされたバリアント (&lt; &gt; &quot;) は偽陽性として除外
    fn confirm_xss(finding: &Finding) -> bool {
        let evidence = finding.evidence.to_lowercase();
        // Require explicit XSS payload match, not generic HTML keywords
        let has_xss_payload = evidence.contains("<script>alert(") ||
            evidence.contains("<img src=x onerror=alert(") ||
            evidence.contains("javascript:alert(") ||
            evidence.contains("<svg onload=alert(");
        has_xss_payload &&
        !evidence.contains("&lt;") &&
        !evidence.contains("&gt;") &&
        !evidence.contains("&quot;")
    }

    //  confirm_cmdi — コマンドインジェクションの確認
    //  Confirms command injection — requires 2 distinct indicators
    //  uid=/gid=/groups= の存在 + /bin/bash 等のシェル参照 + エラーメッセージ
    // ※ 2つ以上の異なる指標が揃った場合のみ確認成功
    fn confirm_cmdi(finding: &Finding) -> bool {
        let evidence = finding.evidence.to_lowercase();
        // Require at least 2 distinct indicators
        let mut hits = 0;
        if (evidence.contains("uid=") && evidence.contains("gid=") && evidence.contains("groups=")) || (evidence.contains("uid=") && evidence.contains("(")) { hits += 1; }
        if evidence.contains("/bin/bash") || evidence.contains("/bin/sh") { hits += 1; }
        if evidence.contains("permission denied") || evidence.contains("command not found") { hits += 1; }

        hits >= 2
    }

    // ※ confirm_lfi — ファイルインクルージョンの確認
    // ※ Confirms LFI — validates /etc/passwd structure
    //  root:x:0:0 または daemon:x: を含みpasswdエントリ形式を検証
    fn confirm_lfi(finding: &Finding) -> bool {
        let evidence = finding.evidence.to_lowercase();
        if !evidence.contains("root:x:0:0") && !evidence.contains("daemon:x:") {
            return false;
        }
        let lines: Vec<&str> = evidence.lines().collect();
        lines.iter().any(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            parts.len() >= 6 && parts[2].parse::<u32>().is_ok() && parts[3].parse::<u32>().is_ok()
        })
    }

    //  confirm_admin_panel — 管理画面アクセスの確認
    //  Confirms admin panel accessibility
    //  URLに /admin /login /dashboard を含みWAFブロックでないことを確認
    fn confirm_admin_panel(finding: &Finding) -> bool {
        let url = finding.url.to_lowercase();
        (url.split('/').any(|p| p == "admin") || url.contains("/login") || url.contains("/dashboard")) &&
        !finding.evidence.is_empty() &&
        !finding.evidence.to_lowercase().contains("cf-ray") &&
        !finding.evidence.to_lowercase().contains("cloudflare")
    }

    //  reduce_false_positive — パイプライン: 全発見結果を確認フィルターに通す
    //  Pipeline: filters all findings through confirmation, retains only true positives
    pub fn reduce_false_positive(findings: Vec<Finding>) -> Vec<Finding> {
        findings
            .into_iter()
            .filter(|f| Self::confirm_vulnerability(f))
            .collect()
    }
}
