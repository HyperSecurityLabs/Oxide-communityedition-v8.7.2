// ----------------------------------------------------------------------------
//  worker.rs — worker thread pool
// ----------------------------------------------------------------------------
//  Worker thread pool — executes scan tasks concurrently with controlled parallelism.
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
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

use crate::cli::args::CliArgs;
use crate::cli::display::ScanBoard;
use crate::core::scanner::ScanResult;
use crate::detection::analyzer::{Analyzer, Finding, Severity};
use crate::http::client::HttpClient;
use crate::http::request::HttpRequest;
use crate::utils::url::UrlUtil;

//  ParallelScanner: ワーカースレッドプール
//  Manages N concurrent worker threads that consume URLs from a shared atomic cursor.
//  Concurrent execution model:
//   - AtomicUsize cursor — 全ワーカーが共有するインデックス ( Ordering::SeqCst)
//   - 各ワーカーはcursor.fetch_add()で次のURLを取得  排他電脳制御不要
//   - ワーカー数はmin(threads, urls.len())でクランプ
//  負荷分散: ラウンドロビン方式でワーカーにフェーズ名を割り当て
//  worker_phases配列でrecon/sqli/xss/lfi/cmdi/crawl/cors/fuzzを巡回
pub struct ParallelScanner {
    client:  Arc<HttpClient>,
    args:    CliArgs,
    workers: usize,
}

impl ParallelScanner {
    pub fn new(client: Arc<HttpClient>, args: CliArgs, workers: usize) -> Self {
        Self { client, args, workers: workers.max(1) }
    }

    //  run: ワーカープール実行
    //  Launches effective worker tasks, each consuming URLs via atomic cursor.
    //  Steps:
    //   1. urlsをArcで共有 / cursorはAtomicUsize
    //   2. effective個のtokio::spawnタスク生成
    //   3. 各タスク: idx = fetch_add  urls[idx]  HTTP GET  analyze  probe_vulns
    //   4. ライブレンダリングループ: 120msごとにScanBoard更新
    //   5. all_findingsをArc<RwLock<Vec<Finding>>>に集約
    //  戻り値はVec<Finding> — 全ワーカーの電脳検出結果を統合
    pub async fn run(&self, urls: Vec<String>, board: Arc<ScanBoard>) -> Vec<Finding> {
        if urls.is_empty() { return Vec::new(); }
        let total     = urls.len();
        let effective = self.workers.min(total).max(1);
        board.set_total(total);
        let urls         = Arc::new(urls);
        let cursor       = Arc::new(AtomicUsize::new(0));
        let all_findings = Arc::new(RwLock::new(Vec::<Finding>::new()));
        let mut handles  = Vec::new();

        let worker_phases = ["recon", "sqli", "xss", "lfi", "cmdi", "crawl", "cors", "fuzz"];

        for wid in 0..effective {
            let client       = self.client.clone();
            let args         = self.args.clone();
            let urls         = urls.clone();
            let cursor       = cursor.clone();
            let board        = board.clone();
            let findings_out = all_findings.clone();

            handles.push(tokio::spawn(async move {
                let analyzer = Analyzer::new();
                loop {
                    let idx = cursor.fetch_add(1, Ordering::SeqCst);
                    if idx >= urls.len() { break; }
                    let url = &urls[idx];
                    let phase = worker_phases[wid % worker_phases.len()];
                    board.worker_start(wid, phase, url).await;
                    match client.send(HttpRequest::get(url)).await {
                        Ok(response) => {
                            let scan_result = ScanResult {
                                url:            url.clone(),
                                status:         response.status,
                                response:       Some(response),
                                payload:        String::new(),
                            };
                            if let Some(finding) = analyzer.analyze(scan_result).await {
                                board.print_finding_live(
                                    &format!("{:?}", finding.severity),
                                    &finding.title, url,
                                ).await;
                                findings_out.write().await.push(finding);
                            }
                            Self::probe_vulns(&client, &args, url, wid, &board, &findings_out).await;
                            board.done.fetch_add(1, Ordering::SeqCst);
                        }
                        Err(e) => { board.worker_error(wid, e.to_string()).await; }
                    }
                }
                board.worker_done(wid, 0).await;
            }));
        }

        let _ = board.render().await;
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(120)).await;
            let h = board.render_height().await;
            if h > 0 { print!("\x1B[{}A\x1B[0G", h); }
            println!("{}", board.render().await);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            if board.done.load(Ordering::SeqCst) >= total { break; }
        }
        let h = board.render_height().await;
        if h > 0 { print!("\x1B[{}A\x1B[0G", h); }
        println!("{}", board.render().await);

        for handle in handles { let _ = handle.await; }
        Arc::try_unwrap(all_findings).map(|rw| rw.into_inner()).unwrap_or_default()
    }

    //  probe_vulns: targeted vulnerability probes — respects --modules isolation
    //  Secondary targeted probes — SQLi, XSS, LFI on all discovered parameters.
    //  Only fires probes for modules the user actually requested.
    async fn probe_vulns(
        client:   &Arc<HttpClient>,
        args:     &CliArgs,
        base_url: &str,
        wid:      usize,
        board:    &Arc<ScanBoard>,
        findings: &Arc<RwLock<Vec<Finding>>>,
    ) {
        let active = args.get_modules();
        let is_all = active.contains(&"all".to_string());
        let probes: &[(&str, &str, &str, &str)] = &[
            ("SQLi", "'",                        "sql syntax", "sqli"),
            ("SQLi", "' OR '1'='1",              "sql syntax", "sqli"),
            ("XSS",  "OXIDEXSS",                 "<script>alert(1)", "xss"),
            ("LFI",  "../../../../etc/passwd",    "root:x:", "lfi"),
            ("LFI",  "..%2F..%2Fetc%2Fpasswd",   "root:x:", "lfi"),
        ];

        let mut params = UrlUtil::extract_query_param_names(base_url);
        if params.is_empty() {
            params = vec![
                "id".into(), "q".into(), "page".into(),
                "file".into(), "url".into(), "name".into(),
                "cat".into(), "dir".into(), "path".into(),
            ];
        }

        for param in &params {
            for &(label, payload, indicator, module_key) in probes {
                // Skip probes for modules not requested
                if !is_all && !active.contains(&module_key.to_string()) { continue; }
                let probe_url = UrlUtil::inject_param(base_url, param, &urlencoding::encode(payload));
                board.worker_start(wid, label, base_url).await;
                if let Ok(resp) = client.send(HttpRequest::get(&probe_url)).await {
                    let body_lower = resp.body.to_lowercase();
                    let payload_reflected = payload.len() > 4 && body_lower.contains(&payload.to_lowercase());
                    let indicator_hit = body_lower.contains(&indicator.to_lowercase());

                    if payload_reflected && indicator_hit {
                        let severity = match label {
                            "SQLi" | "LFI" => Severity::Critical,
                            "XSS" => Severity::High,
                            _ => Severity::Medium,
                        };
                        let finding = Finding::new(
                            base_url, severity,
                            &format!("{} detected", label),
                            &format!("Payload `{}` on param `{}` reflected and triggered `{}`", payload, param, indicator),
                        )
                        .with_evidence(&format!("probe: {}", probe_url))
                        .with_remediation("Sanitize all user-supplied input.");
                        board.print_finding_live(
                            &format!("{:?}", finding.severity), &finding.title, base_url,
                        ).await;
                        findings.write().await.push(finding);
                    }
                }
                if args.rate_limit > 0 {
                    let cap = args.rate_limit.min(1000);
                    let delay_ms = (1000 / cap).max(1);
                    tokio::time::sleep(
                        std::time::Duration::from_millis(delay_ms)
                    ).await;
                }
            }
        }
    }
}
