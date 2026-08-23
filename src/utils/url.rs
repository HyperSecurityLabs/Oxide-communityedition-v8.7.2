// ----------------------------------------------------------------------------
//  url.rs — URL utilities
// ----------------------------------------------------------------------------
//  URL utilities — parsing, normalization, validation for scan targets
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

use strsim::normalized_levenshtein;
use url::Url;

//  UrlUtil — URLユーティリティ / URL parsing, normalization, validation
//   is_valid_url(): validates input is parseable as HTTP/HTTPS URL
//   extract_domain(): extracts host from URL (no scheme)
//   extract_query_param_names(): parses query string param names
//   inject_param(): adds/replaces URL parameter safely
pub struct UrlUtil;

impl UrlUtil {
    //  is_valid_url() — URL検証 / validate URL format
    //   Prepends http:// if scheme is missing
    //   Uses url::Url::parse() for RFC-compliant validation
    //   Returns true if parse succeeds (any valid URL)
    pub fn is_valid_url(input: &str) -> bool {
        let url_str = if input.starts_with("http://") || input.starts_with("https://") {
            input.to_string()
        } else {
            format!("http://{}", input)
        };
        Url::parse(&url_str).is_ok()
    }

    //  extract_domain() — ドメイン抽出 / extract hostname from URL
    //  Returns host string (e.g., "example.com") or empty string on failure
    pub fn extract_domain(url: &Url) -> String {
        url.host_str().unwrap_or("").to_string()
    }

    //  extract_query_param_names() — クエリパラメータ抽出 / parse query param names
    //   Splits query string by &, extracts keys before = sign
    //   Filters empty keys, returns Vec<String>
    pub fn extract_query_param_names(url_str: &str) -> Vec<String> {
        if let Ok(parsed) = Url::parse(url_str) {
            if let Some(query) = parsed.query() {
                if !query.is_empty() {
                    return query.split('&')
                        .filter_map(|param| param.split('=').next().map(|s| s.to_string()))
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }
        }
        Vec::new()
    }

    //  inject_param() — パラメータ注入 / inject or replace URL parameter
    //   If base_url is parseable: removes existing param, appends new pair
    //   If unparseable: appends &param=value or ?param=value as fallback
    //   Returns modified URL string (safe encoding via url::Url)
    //  levenshtein_similarity() — 類似度 / normalised Levenshtein similarity
    //   Returns 0.0 (completely different) to 1.0 (identical)
    //   Uses strsim::normalized_levenshtein for precision accuracy
    pub fn levenshtein_similarity(a: &str, b: &str) -> f64 {
        normalized_levenshtein(a, b)
    }

    //  dedup_urls() — 類似URL重複排除 / remove near-duplicate URLs
    //   Filters out URLs with similarity > threshold to any kept URL
    //   Uses normalised Levenshtein distance for precision comparison
    //   threshold: 0.0 (no dedup) to 1.0 (exact match only)
    pub fn dedup_urls(urls: &[String], threshold: f64) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for url in urls {
            let is_dup = result.iter().any(|kept| {
                Self::levenshtein_similarity(url, kept) > threshold
            });
            if !is_dup {
                result.push(url.clone());
            }
        }
        result
    }

    pub fn inject_param(base_url: &str, param: &str, value: &str) -> String {
        match Url::parse(base_url) {
            Ok(mut url) => {
                let mut pairs: Vec<(String, String)> = url
                    .query_pairs()
                    .filter(|(k, _)| k.as_ref() != param)
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect();
                pairs.push((param.to_string(), value.to_string()));

                let qs = pairs.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                url.set_query(Some(&qs));
                url.to_string()
            }
            Err(_) => {
                if base_url.contains('?') {
                    format!("{}&{}={}", base_url, param, value)
                } else {
                    format!("{}?{}={}", base_url, param, value)
                }
            }
        }
    }
}
