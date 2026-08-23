// ----------------------------------------------------------------------------
//  mod.rs — zero-day detection module
// ----------------------------------------------------------------------------
//  zero-day detection module — ML-based anomaly detection for unknown vulnerabilities
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
//  ゼロデイ電脳検出モジュール — Zero-Day Detection Module Overview
//  This module provides ML-based anomaly detection for discovering unknown
//  vulnerabilities (zero-days) by baseline profiling, feature extraction, and
//  ensemble classification (Random Forest + SVM).
// 
//   features.rs  — 特徴抽出 / HTTP response  ML feature vectors
//   classifier.rs — ML分類器 / Random Forest + SVM classifier
//   baseline.rs   — ベースライン学習 / Statistical profiling of normal responses
//   anomaly.rs    — 異常検知 / Anomaly scoring against baseline
//   engine.rs     — エンジン電脳制御 / Orchestration: baseline  feature  classify
//   trainer.rs    — トレーニング / ML training pipeline
//   standalone.rs — スタンドアロン / Independent zero-day scanner mode
//   hpp.rs        — HPP電脳検出 / HTTP Parameter Pollution testing
// ---------------------------------------------------------------------------

pub mod features;
pub mod classifier;
pub mod baseline;
pub mod anomaly;
pub mod engine;
pub mod trainer;
pub mod standalone;
pub mod hpp;
