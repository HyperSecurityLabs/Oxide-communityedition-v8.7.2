// ----------------------------------------------------------------------------
//  client.rs — Async HTTP client
// ----------------------------------------------------------------------------
//  Async HTTP client built on reqwest — configurable with custom headers,
//  proxies, TLS.
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
//  client.rs — 非同期HTTPクライアント
//  Async HTTP client — reqwestベースの電脳設定可能なクライアント
//  カスタムヘッダープロキシTLS電脳設定UAローテーションをサポート

use anyhow::{Context, Result};
use reqwest::{header::{HeaderName, HeaderValue}, Client, ClientBuilder, cookie::Jar, redirect::Policy};
use std::sync::Arc;
use super::request::HttpRequest;
use super::response::HttpResponse;
use super::useragents::UserAgentPool;

//  HttpClientConfig — HTTPクライアントの電脳設定
//  HTTP client configuration
//  insecure      — TLS証明書検証をスキップ
//  proxy         — プロキシURL
//  user_agent    — カスタムUA (指定時はローテーションより優先)
//  follow_redirects — リダイレクト追従フラグ
//  max_redirects — 最大リダイレクト数
//  cookie        — クッキー文字列
//  jobs          — 同時接続数
#[derive(Clone)]
pub struct HttpClientConfig {
    pub insecure: bool,
    pub proxy: Option<String>,
    pub user_agent: Option<String>,
    pub follow_redirects: bool,
    pub max_redirects: u32,
    pub cookie: Option<String>,
    pub jobs: usize,
}

impl Default for HttpClientConfig {
    fn default() -> Self {
        Self {
            insecure: false,
            proxy: None,
            user_agent: None,
            follow_redirects: true,
            max_redirects: 10,
            cookie: None,
            jobs: 2,
        }
    }
}

//  HttpClient — the dating profile of HTTP requests
//  Main HTTP client — wraps reqwest Client with UA pool
//  client     — reqwest Client (the muscle)
//  ua_pool    — User-Agent pool (the disguises)
//  user_agent — fixed UA (if you're boring)
//  cookie_str — cookie string (the sweet nothings)
#[derive(Clone)]
pub struct HttpClient {
    client:     Client,
    ua_pool:    UserAgentPool,
    user_agent: Option<String>,
    cookie_str: Option<String>,
    ///  グローバルリクエスト計数機 / global request ticker.
    ///  Live progress boards poll this so % moves with every request,
    ///  not only when a full URL finishes.
    pub req_counter: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

//  クライアント実装 — 構築・送信の全ライフサイクル
//  Client implementation — full build/send lifecycle
impl HttpClient {
    //  new — 電脳設定からクライアントを構築
    //  Builds HttpClient from config (calls build_client internally)
    pub fn new(config: HttpClientConfig) -> Result<Self> {
        let client = Self::build_client(&config)?;
        Ok(Self {
            client,
            ua_pool:  UserAgentPool::full(),
            user_agent: config.user_agent,
            cookie_str: config.cookie,
            req_counter: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        })
    }

    //  build_client — reqwest ClientBuilder の電脳設定
    //  Builds reqwest Client with cert validation, proxy, cookies, redirects
    //  証明書検証スキップ / タイムアウト / クッキーストア / プロキシ
    fn build_client(config: &HttpClientConfig) -> Result<Client> {
        let mut builder = ClientBuilder::new()
            .danger_accept_invalid_certs(config.insecure)
            .timeout(std::time::Duration::from_secs(30))
            .cookie_store(true)
            .pool_max_idle_per_host(config.jobs.max(8))
            .tcp_keepalive(std::time::Duration::from_secs(30));

        if let Some(ref cookie_str) = config.cookie {
            let jar = Jar::default();
            for pair in cookie_str.split(';') {
                let parts: Vec<&str> = pair.splitn(2, '=').collect();
                if parts.len() == 2 {
                    jar.add_cookie_str(
                        &format!("{}={}", parts[0].trim(), parts[1].trim()),
                        &"http://localhost".parse().expect("valid static URL"),
                    );
                }
            }
            builder = builder.cookie_provider(Arc::new(jar));
        }

        if let Some(ref proxy_url) = config.proxy {
            let parsed = reqwest::Proxy::all(proxy_url)
                .with_context(|| format!("Invalid proxy URL: {}", proxy_url))?;
            builder = builder.proxy(parsed);

            let cert = reqwest::Certificate::from_der(crate::BURP_CA_CERT)
                .ok()
                .or_else(|| {
                    let text = String::from_utf8_lossy(crate::BURP_CA_CERT);
                    reqwest::Certificate::from_pem(text.as_bytes()).ok()
                });
            if let Some(c) = cert {
                builder = builder.add_root_certificate(c);
            }
        }

        builder = builder.redirect(Policy::limited(config.max_redirects as usize));

        builder
            .build()
            .with_context(|| "Failed to build HTTP client")
    }

    pub fn cookie_string(&self) -> Option<&str> {
        self.cookie_str.as_deref()
    }

    //  send — the full send. literally.
    //  Full request lifecycle: disguise yourself, look pretty, fire, pray
    //  1. Pick a UA (be who you wanna be today)
    //  2. Match Accept headers to UA (WAFs notice when you dress wrong)
    //  3. Apply custom headers (no overwriting the disguise)
    //  4. Attach body (optional pillow talk)
    //  5. SEND IT
    //  6. Convert response (judge the book by its cover)
    pub async fn send(&self, request: HttpRequest) -> Result<HttpResponse> {
        use std::sync::atomic::Ordering;
        self.req_counter.fetch_add(1, Ordering::Relaxed);
        let ua = self.user_agent.as_deref().unwrap_or_else(|| self.ua_pool.next());
        let (accept, accept_lang, accept_enc) = UserAgentPool::accept_headers_for(ua);

        let mut req = self.client.request(request.method.clone(), request.url.as_str());

        // Apply custom headers from request first, then override critical headers
        // with pool-selected values so UA rotation works correctly.
        for (key, value) in &request.headers {
            let key_lower = key.to_lowercase();
            match key_lower.as_str() {
                "user-agent" | "accept" | "accept-language" | "accept-encoding" => {}
                _ => {
                    // skip invalid header pairs instead of panicking
                    if let (Ok(hk), Ok(hv)) = (HeaderName::from_bytes(key.as_bytes()), HeaderValue::from_str(value)) {
                        req = req.header(hk, hv);
                    }
                }
            }
        }

        req = req
            .header("User-Agent",      ua)
            .header("Accept",          accept)
            .header("Accept-Language", accept_lang)
            .header("Accept-Encoding", accept_enc);

        if let Some(body) = &request.body {
            req = req.body(body.clone());
        }
        if let Some(ref cookie) = self.cookie_str {
            if let Ok(hv) = HeaderValue::from_str(cookie.as_str()) {
                req = req.header("Cookie", hv);
            }
        }
        let response = req
            .send()
            .await
            .with_context(|| format!("Failed to send request to {}", request.url))?;
        HttpResponse::from_reqwest(response).await
    }

    //  get — the easy button for GET requests
    //  Convenience GET request (because who doesn't like easy?)
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.send(HttpRequest::get(url)).await
    }

    //  post — when you need to send something intimate to the server
    //  Convenience POST request (sending your love letters to APIs)
    pub async fn post(&self, url: &str, body: &str) -> Result<HttpResponse> {
        self.send(HttpRequest::post(url, body)).await
    }
}
