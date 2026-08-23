// ----------------------------------------------------------------------------
//  headers.rs — HTTP header manipulation
// ----------------------------------------------------------------------------
//  HTTP header manipulation — common security header checks and custom header
//  injection.
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
//  headers.rs — HTTPヘッダー操作
//  HTTP header manipulation — security header audit + custom header injection
//  セキュリティヘッダーの有無をチェック (HSTS, CSP, XFO等)

use std::collections::HashMap;

//  Headers — カスタムHTTPヘッダー管理
//  Custom HTTP header management
#[derive(Clone, Debug)]
pub struct Headers {
    headers: HashMap<String, String>,
}

//  ヘッダー実装
//  Headers implementation
impl Headers {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    //  default_headers — 標準リクエストヘッダー
    //  Default request headers (Connection: keep-alive, Upgrade-Insecure-Requests: 1)
    pub fn default_headers() -> Self {
        let mut headers = Self::new();
        headers.add("Connection", "keep-alive");
        headers.add("Upgrade-Insecure-Requests", "1");
        headers
    }

    pub fn add(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn to_hashmap(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    //  audit_security_headers — セキュリティヘッダーの完全監査
    //  Full security header audit
    //  HSTS  HTTP Strict-Transport-Security
    //  CSP   Content-Security-Policy (XSS対策)
    //  XCTO  X-Content-Type-Options (MIMEスニッフィング対策)
    //  XFO   X-Frame-Options (クリックジャッキング対策)
    //  XXSSP  X-XSS-Protection (レガシーブラウザ対策)
    //  Referrer  Referrer-Policy
    //  Permissions  Permissions-Policy
    //  CORS  Access-Control-Allow-Origin
    //  SecureCookie  Set-Cookie の有無
    pub fn audit_security_headers(response_headers: &HashMap<String, String>) -> Vec<(String, String, String)> {
        let checks = [
            ("Strict-Transport-Security", "HSTS", "Missing HTTP Strict-Transport-Security header"),
            ("Content-Security-Policy", "CSP", "Missing Content-Security-Policy header — XSS risk"),
            ("X-Content-Type-Options", "XCTO", "Missing X-Content-Type-Options: nosniff"),
            ("X-Frame-Options", "XFO", "Missing X-Frame-Options — clickjacking risk"),
            ("X-XSS-Protection", "XXSSP", "Missing X-XSS-Protection header"),
            ("Referrer-Policy", "Referrer", "Missing Referrer-Policy header"),
            ("Permissions-Policy", "Permissions", "Missing Permissions-Policy header"),
            ("Access-Control-Allow-Origin", "CORS", "Missing CORS headers"),
            ("Set-Cookie", "SecureCookie", "No Set-Cookie header found"),
        ];

        let mut results = Vec::new();
        for (header, short, desc) in &checks {
            if *header == "X-XSS-Protection" {
                if let Some(val) = response_headers.get("x-xss-protection") {
                    if val == "0" || val == "1" {
                        results.push((short.to_string(), "present".to_string(), "X-XSS-Protection header set".to_string()));
                        continue;
                    }
                }
            }

            let header_lower = header.to_lowercase();
            let found = response_headers.keys().any(|k| k.to_lowercase() == header_lower);

            if found {
                results.push((short.to_string(), "present".to_string(), desc.replacen("Missing ", "Found ", 1)));
            } else {
                results.push((short.to_string(), "missing".to_string(), desc.to_string()));
            }
        }
        results
    }

}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}
