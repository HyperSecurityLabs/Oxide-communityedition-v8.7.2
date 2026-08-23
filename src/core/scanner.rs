// ----------------------------------------------------------------------------
//  scanner.rs — base scanner trait
// ----------------------------------------------------------------------------
//  Base scanner trait — defines the common interface all vulnerability scanners implement.
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

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::cli::args::CliArgs;
use crate::cli::spinner::Spinner;
use crate::http::client::HttpClient;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::payload::generator::PayloadGenerator;

//  Scanner: ベーススキャナ構造体
//  Core scanner — sends HTTP requests with payloads and sends results to analyzer.
//  Contract (scan method):
//   1. generate_paths() — URLパスにペイロードを注入
//   2. generate_params() — クエリパラメータにペイロードを注入
//   3. generate_headers() — HTTPヘッダーにペイロードを注入
//   4. 各結果をScanResultにラップ  mpscチャネルで送信
//  scan()メソッドがScannerトレイトの基本契約を定義
// ※ ScanResult: url + status + response + payload を1パケットとして転送
pub struct Scanner {
    client: Arc<HttpClient>,
    args: CliArgs,
    payload_gen: PayloadGenerator,
    tx: Sender<ScanResult>,
}

#[derive(Clone, Debug)]
pub struct ScanResult {
    pub url: String,
    pub status: u16,
    pub response: Option<HttpResponse>,
    pub payload: String,
}

impl Scanner {
    pub fn new(
        client: Arc<HttpClient>,
        args: CliArgs,
        payload_gen: PayloadGenerator,
        tx: Sender<ScanResult>,
    ) -> Self {
        Self {
            client,
            args,
            payload_gen,
            tx,
        }
    }

    //  scan: 電脳走査実行電脳入口点
    //  Entry point — iterates all payload vectors (paths, params, headers).
    //  3 scanning phases:
    //   ➊ scan_paths — directory/file brute-force with payload paths
    //   ➋ scan_params — query string parameter fuzzing
    //   ➌ scan_headers — HTTP header injection testing
    //  各フェーズは独立して実行結果はすべてtxチャネルに送信
    pub async fn scan(&self) -> Result<()> {
        let paths = self.payload_gen.generate_paths();
        let params = self.payload_gen.generate_params();
        let headers = self.payload_gen.generate_headers();

        self.scan_paths(&paths).await?;
        self.scan_params(&params).await?;
        self.scan_headers(&headers).await?;

        Ok(())
    }

    //  scan_paths: パスベースの電脳走査
    //  Scans URL paths by appending each payload to the target URL.
    //  Flow: for each path  GET request  wrap in ScanResult  send via tx
    //  スピナー表示あり / エラーはeprintlnで記録
    async fn scan_paths(&self, paths: &[String]) -> Result<()> {
        let spinner = Spinner::path_spinner();

        for path in paths {
            let url = format!("{}{}", self.args.target_url(), path);
            let request = HttpRequest::get(&url);

            match self.client.send(request).await {
                Ok(response) => {
                    let result = ScanResult {
                        url: url.clone(),
                        status: response.status,
                        response: Some(response),
                        payload: path.clone(),
                    };
                    if self.tx.send(result).await.is_err() {
                        // Channel closed — results will be collected via other paths
                    }
                }
                Err(e) => {
                    eprintln!("  [!] Request failed on {}: {}", url, e);
                }
            }

            let _ = spinner.next();
        }

        Ok(())
    }

    //  scan_params: パラメータベースの電脳走査
    //  Scans query parameters by appending ?payload to the target URL.
    //  クエリ文字列インジェクションをテスト
    async fn scan_params(&self, params: &[String]) -> Result<()> {
        let spinner = Spinner::param_spinner();

        for param in params {
            let url = format!("{}?{}", self.args.target_url(), param);
            let request = HttpRequest::get(&url);

            match self.client.send(request).await {
                Ok(response) => {
                    let result = ScanResult {
                        url: url.clone(),
                        status: response.status,
                        response: Some(response),
                        payload: param.clone(),
                    };
                    if self.tx.send(result).await.is_err() {
                        // Channel closed — results will be collected via other paths
                    }
                }
                Err(e) => {
                    eprintln!("  [!] Param scan request failed for {}: {}", url, e);
                }
            }

            let _ = spinner.next();
        }

        Ok(())
    }

    pub fn generate_payloads(&self) -> Vec<String> {
        self.payload_gen.generate_paths()
    }

    //  scan_headers: ヘッダーベースの電脳走査
    //  Scans HTTP headers by injecting payloads into header values.
    //  ヘッダーインジェクション / HTTPヘッダースプリッティングをテスト
    //  payloadは "Key: Value" 形式でsplit_once(':')で解析
    async fn scan_headers(&self, headers: &[String]) -> Result<()> {
        let spinner = Spinner::header_spinner();

        for header_str in headers {
            let mut request = HttpRequest::get(self.args.target_url());

            if let Some((key, value)) = header_str.split_once(':') {
                request.add_header(key.trim(), value.trim());
            }

            match self.client.send(request).await {
                Ok(response) => {
                    let result = ScanResult {
                        url: self.args.target_url().to_string(),
                        status: response.status,
                        response: Some(response),
                        payload: header_str.clone(),
                    };
                    if self.tx.send(result).await.is_err() {
                        // Channel closed — results will be collected via other paths
                    }
                }
                Err(e) => {
                    eprintln!("  [!] Header scan request failed: {}", e);
                }
            }

            let _ = spinner.next();
        }

        Ok(())
    }

    //  scan_body: リクエストボディ電脳走査
    //  Sends payloads as POST body content for parameter/body injection testing.
    //  POSTリクエストのボディにペイロードを電脳設定して送信
    //  HTTP POST + raw body の両方をテスト
    pub async fn scan_body(&self, payloads: &[String]) -> Result<Vec<crate::detection::analyzer::Finding>> {
        use crate::detection::analyzer::{Finding, Severity};
        let mut findings = Vec::new();
        for payload in payloads {
            let url = format!("{}", self.args.target_url());

            let request = HttpRequest::post(&url, payload);

            match self.client.send(request).await {
                Ok(response) => {
                    let body_text = &response.body;
                    let is_error_like = response.status >= 400
                        || body_text.to_lowercase().contains("error")
                        || body_text.to_lowercase().contains("exception")
                        || body_text.to_lowercase().contains("stack trace")
                        || body_text.to_lowercase().contains("syntax");

                    if is_error_like && !body_text.is_empty() {
                        findings.push(Finding {
                            url: url.clone(),
                            title: format!("Body injection response (HTTP {})", response.status),
                            severity: Severity::Medium,
                            evidence: format!("Payload '{}' triggered HTTP {}", payload, response.status),
                            description: "POST body payload triggered an error response. Possible injection vector.".to_string(),
                            remediation: "Validate and sanitize all POST body inputs server-side".to_string(),
                        });
                    }

                    let result = ScanResult {
                        url: url.clone(),
                        status: response.status,
                        response: Some(response),
                        payload: payload.clone(),
                    };
                    if self.tx.send(result).await.is_err() {
                        // Channel closed — results will be collected via other paths
                    }
                }
                Err(e) => {
                    eprintln!("  [!] Body scan request failed: {}", e);
                }
            }
        }
        Ok(findings)
    }
}
