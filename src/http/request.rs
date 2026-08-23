// ----------------------------------------------------------------------------
//  request.rs — Raw HTTP request builder
// ----------------------------------------------------------------------------
//  Raw HTTP request builder — constructs requests with full control over
//  method, headers, body.
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
//  request.rs — HTTPリクエストビルダー
//  HTTP request builder — full control over method, headers, body
//  URL検証メソッド電脳設定カスタムヘッダー注入サイズ計算

use anyhow::Result;
use reqwest::Method;
use std::collections::HashMap;
use std::str::FromStr;
// Check the target url Specified 
use url::Url;

use super::headers::Headers;

//  HttpRequest — 完全なHTTPリクエスト表現
//  Full HTTP request representation
//  url     — リクエスト先URL (urlクレートで検証済み)
//  method — HTTPメソッド (GET/POST等)
//  headers — カスタムヘッダー
//  body   — リクエストボディ (POST時)
#[derive(Clone, Debug)]
pub struct HttpRequest {
    pub url: Url,
    pub method: Method,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

//  リクエストビルダーの実装
//  Request builder implementation
impl HttpRequest {
    //  new — URLをパースしてGETリクエストを作成
    //  Creates GET request with parsed URL, default headers
    pub fn new(url: &str) -> Result<Self> {
        let parsed_url = Url::parse(url)?;

        Ok(Self {
            url: parsed_url,
            method: Method::GET,
            headers: Headers::default_headers().to_hashmap(),
            body: None,
        })
    }

    //  get — GETリクエストの簡易ビルダー (パース失敗時はlocalhostにフォールバック)
    //  GET request builder with fallback
    pub fn get(url: &str) -> Self {
        Self::new(url).unwrap_or_else(|_| {
            let fallback_url = Url::parse("http://localhost").unwrap_or_else(|_| {
                Url::from_str("http://localhost:80").expect("hardcoded fallback URL always valid")
            });
            Self {
                url: fallback_url,
                method: Method::GET,
                headers: Headers::default_headers().to_hashmap(),
                body: None,
            }
        })
    }

    //  post — POSTリクエストの簡易ビルダー
    //  POST request builder with body
    pub fn post(url: &str, body: &str) -> Self {
        let mut req = Self::get(url);
        req.method = Method::POST;
        req.body = Some(body.to_string());
        req.headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        req
    }

    //  add_header — カスタムヘッダーを追加
    //  Adds custom header to request — strips CRLF to prevent header injection
    pub fn add_header(&mut self, key: &str, value: &str) {
        let safe_key = key.replace('\r', "").replace('\n', "");
        let safe_value = value.replace('\r', "").replace('\n', "");
        if !safe_key.is_empty() {
            self.headers.insert(safe_key, safe_value);
        }
    }

    //  size_bytes — リクエストのおおよそのサイズを計算
    //  Calculates approximate request size (method line + headers + body)
    pub fn size_bytes(&self) -> u64 {
        let method_line = self.method.as_str().len() + self.url.as_str().len() + 11;
        let headers_size: usize = self.headers.iter()
            .map(|(k, v)| k.len() + 2 + v.len() + 2)
            .sum();
        let body_size = self.body.as_ref().map(|b| b.len()).unwrap_or(0);
        (method_line + headers_size + body_size + 2) as u64
    }

}
