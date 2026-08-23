// ----------------------------------------------------------------------------
//  mod.rs — AI module root
// ----------------------------------------------------------------------------
//  AI module root — ML-powered exploit analysis, pattern learning, payload mutation
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
//  AI Module Overview / AIモジュール概要
//  This module provides ML-powered intelligence for adaptive exploitation:
//    exploit_analyzer  — learns from response patterns to identify exploitable paths
//    response_analyzer — parses HTTP responses for ML feature extraction
//    payload_mutator   — AI-driven payload mutation (8 strategies + WAF bypass)
//    pattern_learner   — Bayesian pattern learning with exponential moving average
//
//  The AI pipeline: response  feature extraction  pattern learning  payload mutation
// AI-powered exploit analysis and adaptive testing
pub mod exploit_analyzer;
pub mod response_analyzer;
pub mod payload_mutator;
pub mod pattern_learner;
