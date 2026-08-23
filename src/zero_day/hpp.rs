// ----------------------------------------------------------------------------
//  hpp.rs — HTTP parameter pollution detector
// ----------------------------------------------------------------------------
//  HTTP parameter pollution detector — tests parameter handling for parsing inconsistencies
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

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use url::Url;

use crate::cli::display::{
    TSUYUKUSA, HISUI, FUJI,
};
use crate::detection::analyzer::{Finding, Severity};
use crate::http::client::HttpClient;
use crate::http::request::HttpRequest;

fn tc(s: &str, c: (u8, u8, u8)) -> String {
    s.truecolor(c.0, c.1, c.2).to_string()
}

//  HPP_PAYLOADS — HPPテストペイロード一覧 / HPP test payload variants
//  Tests duplicate parameter handling with int, str, empty, SQLi, XSS,
//  special chars, JSON, and array values.
const HPP_PAYLOADS: &[(&str, &str)] = &[
    ("dup-int",   "42"),
    ("dup-str",   "hpp_test"),
    ("dup-empty", ""),
    ("dup-sql",   "' OR '1'='1"),
    ("dup-xss",   "<script>"),
    ("dup-special", "!@#$%^&*()"),
    ("dup-json",  "{\"a\":1}"),
    ("dup-array", "[]"),
];

//  HppDetector — HTTPパラメータ汚染電脳検出
//  Tests if servers accept duplicate parameters and which value they use.
//   scan(): iterates URLs with query params, sends duplicate-parameter
//    requests with various payloads, compares response body delta + status.
//   build_hpp_url(): appends &param=poison to original URL.
//   Detection: significant body delta (>256B) or status change = anomaly.
pub struct HppDetector;

impl HppDetector {
    //  scan() — HPP電脳走査フロー / HTTP Parameter Pollution scan
    //   Filters URLs with query parameters (has_query_params)
    //   For each candidate URL:
    //    1. Sends base request to get baseline (body length, status)
    //    2. Parses original parameters via url::Url::query_pairs
    //    3. For each param  each payload variant:
    //       - Builds HPP URL (appends &param=poison)
    //       - Sends request, compares body delta and status
    //       - Significant = Δbody > 256B OR status change
    //    4. If vulnerabilities found: creates Finding with severity High
    //   Returns HppSummary: vulnerable_count, total_anomalies, timeouts
    pub async fn scan(client: &Arc<HttpClient>, urls: &[String], findings: &mut Vec<Finding>) -> Result<HppSummary> {
        let mut summary = HppSummary::default();
        let candidates: Vec<&String> = urls.iter().filter(|u| has_query_params(u)).collect();

        if candidates.is_empty() {
            println!("  {} No URLs with query parameters found — skipping HPP scan",
                tc("!", HISUI));
            return Ok(summary);
        }

        println!("  {} HTTP Parameter Pollution scan: {} URL candidates",
            tc("▸", TSUYUKUSA),
            tc(&candidates.len().to_string(), FUJI));

        for (idx, url) in candidates.iter().enumerate() {
            if crate::is_shutdown_requested() {
                break;
            }

            let base_req = HttpRequest::get(url);
            let base_resp = match tokio::time::timeout(Duration::from_secs(8), client.send(base_req)).await {
                Ok(Ok(r)) => r,
                _ => continue,
            };
            let base_len = base_resp.body.len();

            let parsed = match Url::parse(url) {
                Ok(u) => u,
                Err(_) => continue,
            };
            let params: Vec<(String, String)> = parsed.query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect();

            if params.is_empty() {
                continue;
            }

            let mut vulnerable_params = Vec::new();

            for (param_name, original_val) in &params {
                for (variant_label, poison_val) in HPP_PAYLOADS {
                    let hpp_url = build_hpp_url(url, param_name, original_val, poison_val);
                    let req = HttpRequest::get(&hpp_url);

                    match tokio::time::timeout(Duration::from_secs(8), client.send(req)).await {
                        Ok(Ok(resp)) => {
                            summary.total_requests += 1;
                            let delta = (resp.body.len() as i64 - base_len as i64).unsigned_abs();
                            let significant = delta > 256 || resp.status != base_resp.status;

                            if significant {
                                summary.total_anomalies += 1;
                                vulnerable_params.push((
                                    param_name.clone(),
                                    variant_label.to_string(),
                                    resp.status,
                                    delta,
                                ));
                            }
                        }
                        _ => {
                            summary.timeouts += 1;
                        }
                    }
                }
            }

            if !vulnerable_params.is_empty() {
                summary.vulnerable_count += 1;
                let details: Vec<String> = vulnerable_params.iter()
                    .map(|(p, v, status, delta)| {
                        format!("{} via {} | status={} | Δbody={}B", p, v, status, delta)
                    })
                    .collect();

                let finding = Finding::new(
                    url,
                    Severity::High,
                    "[HPP] HTTP Parameter Pollution",
                    &format!(
                        "Parameter pollution detected on {} parameter(s). \
                         Server behaviour changes when duplicate parameters are sent. \
                         {}",
                        vulnerable_params.len(),
                        details.join("; "),
                    ),
                )
                .with_evidence(&format!("Parameters affected: {}",
                    vulnerable_params.iter().map(|(p, _, _, _)| p.as_str()).collect::<Vec<_>>().join(", ")))
                .with_remediation(
                    "Reject requests with duplicate parameters. \
                     Use the last-occurring value consistently. \
                     Implement input validation on server-side."
                );

                findings.push(finding);
                println!("  {} [HPP] {} — {} parameter(s) vulnerable",
                    tc("⚠", HISUI),
                    tc(&truncate_url(url, 60), FUJI),
                    tc(&vulnerable_params.len().to_string(), HISUI));
            }

            if idx % 5 == 0 {
                print!("\r  {} Progress: {}/{} URLs  |  anomalies: {}  |  vuln URLs: {}   ",
                    tc("▸", TSUYUKUSA),
                    tc(&(idx + 1).to_string(), HISUI),
                    tc(&candidates.len().to_string(), FUJI),
                    tc(&summary.total_anomalies.to_string(), HISUI),
                    tc(&summary.vulnerable_count.to_string(), HISUI));
                use std::io::{Write, stdout};
                stdout().flush().ok();
            }
        }

        println!();
        println!("  {} HPP scan complete — {} vulnerable URLs, {} anomalies, {} timeouts",
            tc("✓", HISUI),
            tc(&summary.vulnerable_count.to_string(), HISUI),
            tc(&summary.total_anomalies.to_string(), HISUI),
            tc(&summary.timeouts.to_string(), FUJI));

        Ok(summary)
    }
}

fn has_query_params(url: &str) -> bool {
    url.contains('?')
}

//  build_hpp_url() — HPPテストURL構築 / builds duplicate-parameter URL
//  Strips fragment (#), appends &param=poison to the base URL
//  Original value is kept as-is; the duplicate param triggers parser ambiguity.
fn build_hpp_url(base_url: &str, param: &str, _orig_val: &str, poison: &str) -> String {
    let fragment_stripped = match base_url.find('#') {
        Some(pos) => &base_url[..base_url.floor_char_boundary(pos)],
        None => base_url,
    };
    format!("{}&{}={}", fragment_stripped, param, poison)
}

fn truncate_url(url: &str, max: usize) -> String {
    if url.len() > max {
        let end = url.floor_char_boundary(max.saturating_sub(3));
        format!("{}...", &url[..end])
    } else {
        url.to_string()
    }
}

#[derive(Default, Debug)]
pub struct HppSummary {
    pub vulnerable_count: usize,
    pub total_anomalies: usize,
    pub total_requests: usize,
    pub timeouts: usize,
}
