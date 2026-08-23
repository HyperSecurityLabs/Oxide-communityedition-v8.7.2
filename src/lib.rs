// ----------------------------------------------------------------------------
//  lib.rs — library root with public API re-exports
// ----------------------------------------------------------------------------
//  Library crate root that declares all public modules (advanced, ai, cli, core,
//  detection, http, payload, report, scanner, utils) and re-exports key types
//  (CliArgs, ScanEngine, Analyzer, Finding, HttpClient, PayloadGenerator,
//  ReportGenerator) for external consumers. Also defines the global SHUTDOWN
//  atomic flag used for graceful Ctrl+C/SIGTERM handling across the async
//  runtime.
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & visual design
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
//  ライブラリルートモジュール / library root module
//  全公開モジュールと型を再エクスポート / re-exports all public modules & types
//
pub mod advanced;
pub mod ai;
pub mod cli;
pub mod core;
pub mod db;
pub mod detection;
pub mod http;
pub mod payload;
pub mod session_hijack;
pub mod report;
pub mod scanner;
pub mod utils;

//  シャットダウンフラグ / global shutdown flag — set by SIGINT/SIGTERM handlers
//  全非同期ループがこのフラグをポーリングして安全に終了 / all async loops poll this for safe exit
use std::sync::atomic::{AtomicBool, Ordering};

// The "oh shit" button — Ctrl+C sets this and we gracefully GTFO
pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

//  シャットダウン要求確認 / check if shutdown was requested
pub fn is_shutdown_requested() -> bool {
    SHUTDOWN.load(Ordering::Acquire)
}



pub use cli::args::CliArgs;
pub use core::engine::ScanEngine;
pub use detection::analyzer::{Analyzer, Finding, Severity};
pub use http::client::HttpClient;
pub use payload::generator::PayloadGenerator;
pub use report::generator::ReportGenerator;

//  ライブラリ定数 / library constants — version, name, description
pub const VERSION: &str = "8.7.2-community-edition";
pub const NAME: &str = "OXIDE Community Edition";
pub const DESCRIPTION: &str = "Open eXtensible Intelligence & Detection Engine — Community Edition";

// Burp's dirty little secret — embedded cert so we can peek through the proxy
// without TLS throwing a tantrum. Yes, we're that intimate with PortSwigger.
pub const BURP_CA_CERT: &[u8] = include_bytes!("oxide.crt");
