// ----------------------------------------------------------------------------
//  mod.rs — Advanced OXIDE features
// ----------------------------------------------------------------------------
//  Advanced OXIDE features — API fuzzing, clustering, JS crawling, ML detection
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
//  Advanced Module Overview / 高度なモジュール概要
//  This module aggregates OXIDE's advanced scanning subsystems:
//    api_fuzzer    — REST/GraphQL API fuzzing with injection templates
//    cache         — response caching layer (memory + disk with LRU eviction)
//    cluster       — distributed scan coordination (master/agent architecture)
//    crawler_js    — JS-aware crawling (SPA support, URL extraction)
//    evasion       — WAF/IDS bypass techniques (encoding, header manipulation)
//    ml_detector   — ML-based anomaly detection (statistical + neural)
//    plugin        — dynamic plugin loading via FFI (.so/.dll/.dylib)
//    rate_limiter  — token-bucket algorithm + adaptive rate adjustment
//    session       — multi-phase scan session state management
//    websocket     — WebSocket fuzzing (handshake, frame injection, auth bypass)
//
//  Each module is self-contained and communicates via OXIDE's shared types.
/// Advanced OXIDE features for v8
/// Includes distributed scanning, advanced evasion, ML detection, and more

pub mod api_fuzzer;
pub mod cache;
pub mod cluster;
pub mod crawler_js;
pub mod evasion;
pub mod ml_detector;
pub mod plugin;
pub mod rate_limiter;
pub mod session;
pub mod websocket;
