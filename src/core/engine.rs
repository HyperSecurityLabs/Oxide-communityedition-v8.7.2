// ----------------------------------------------------------------------------
//  engine.rs — scan engine
// ----------------------------------------------------------------------------
//  Scan engine — main execution loop, manages scanner lifecycle and error handling.
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
use tokio::sync::mpsc;

use crate::cli::args::CliArgs;
use crate::http::client::{HttpClient, HttpClientConfig};
use crate::payload::generator::PayloadGenerator;
use crate::detection::analyzer::Analyzer;
use crate::report::generator::ReportGenerator;

//  ScanEngine: 電脳走査エンジンのライフサイクル
//  Main scan lifecycle — initialization  payload generation  scan  analysis  report.
//  Lifecycle:
//   1. new() — HTTPクライアント・ペイロード生成器・分析器を初期化
//   2. run() — mpscチャネル経由で電脳走査結果をストリーム処理
//   3. analyze_task — 結果をAnalyzeFinding生成レポート保存
//  エラーハンドリング: anyhow::Resultで統一 / リトライロジックはscanner側
//  電脳走査が中断されてもpartial resultを保持する設計
pub struct ScanEngine {
    args: CliArgs,
    client: Arc<HttpClient>,
    payload_gen: PayloadGenerator,
    analyzer: Analyzer,
    reporter: ReportGenerator,
}

impl ScanEngine {
    pub fn new(args: CliArgs) -> Result<Self> {
        let http_config = HttpClientConfig {
            insecure: args.insecure || args.burp,
            proxy: if args.burp { Some("http://127.0.0.1:8080".to_string()) } else { None },
            user_agent: args.user_agent.clone(),
            follow_redirects: true,
            max_redirects: args.max_redirects,
            cookie: args.cookie.clone(),
            jobs: args.jobs,
        };
        let client = HttpClient::new(http_config)
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {}", e))?;
        let client = Arc::new(client);

        let payload_gen = PayloadGenerator::new();
        let analyzer = Analyzer::new();
        let reporter = ReportGenerator::new(&args.format, "", "", &[], 0)
            .with_modules(args.get_modules());

        Ok(Self {
            args,
            client,
            payload_gen,
            analyzer,
            reporter,
        })
    }

    //  run: メイン電脳走査ループ
    //  Main scan loop with concurrent analysis pipeline.
    //  Steps:
    //   1. Create mpsc channel (capacity=100) for scan results
    //   2. Spawn Scanner instance with payload generator
    //   3. Spawn analyze_task — consumes results from channel
    //   4. Scanner.scan() executes all payload probes
    //   5. Wait for analyze_task to finish  reports saved
    //  チャネルを介したプロデューサー/コンシューマーパターン
    //  エラーが発生してもanalyze_taskは中断しない
    pub async fn run(&self) -> Result<()> {
        println!("Starting scan on: {}", self.args.target_url());
        println!("Threads: {}", self.args.threads);
        println!();

        let (tx, mut rx) = mpsc::channel(100);

        let scanner = crate::core::scanner::Scanner::new(
            self.client.clone(),
            self.args.clone(),
            self.payload_gen.clone(),
            tx,
        );

        let analyze_task = tokio::spawn({
            let analyzer = self.analyzer.clone();
            let output_path = self.args.output.clone();
            let mut reporter = self.reporter.clone();
            
            async move {
                while let Some(result) = rx.recv().await {
                    let finding = analyzer.analyze(result).await;
                    if let Some(finding) = finding {
                        reporter.add_finding(finding);
                    }
                }
                
                if let Some(path) = output_path {
                    let p = std::path::PathBuf::from(&path);
                    if let Err(e) = reporter.save(&p) {
                        eprintln!("  [!] Failed to save report: {}", e);
                    }
                }
            }
        });

        scanner.scan().await?;
        
        let _ = analyze_task.await;

        Ok(())
    }
}
