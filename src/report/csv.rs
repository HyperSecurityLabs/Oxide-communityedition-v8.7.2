// ----------------------------------------------------------------------------
//  csv.rs — CSV report generator
// ----------------------------------------------------------------------------
//  CSV report generator — exports findings in comma-separated format for spreadsheet analysis.
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

//  CsvReport: CSVレポート生成器
//  Exports findings as comma-separated values for spreadsheet/BI tool analysis.
//  カラム構成: URL, Severity, Title, Description, Evidence, Remediation
//  エスケープ処理: カンマ/引用符/改行を含むフィールドは二重引用符でラップ
//  escape_field: 特殊文字を含むフィールドを安全にCSVフォーマット
pub struct CsvReport;

impl CsvReport {
    //  generate_header: CSVヘッダー行生成
    //  Returns the CSV column header line.
    pub fn generate_header() -> String {
        "URL,Severity,Title,Description,Evidence,Remediation\n".to_string()
    }

    pub fn escape_field(field: &str) -> String {
        let needs_quotes = field.contains(',') || field.contains('"') || field.contains('\n');
        
        if needs_quotes {
            let escaped = field.replace('"', "\"\"");
            format!("\"{}\"", escaped)
        } else {
            field.to_string()
        }
    }

    //  generate_row: CSVデータ行生成
    //  Formats a single finding as a CSV row with proper escaping.
    //  ヘルパー: escape_field — カンマ/引用符/改行を処理
    pub fn generate_row(
        url: &str,
        severity: &str,
        title: &str,
        description: &str,
        evidence: &str,
        remediation: &str,
    ) -> String {
        format!(
            "{},{},{},{},{},{}\n",
            Self::escape_field(url),
            Self::escape_field(severity),
            Self::escape_field(title),
            Self::escape_field(description),
            Self::escape_field(evidence),
            Self::escape_field(remediation)
        )
    }
}
