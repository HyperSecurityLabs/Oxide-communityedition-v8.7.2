// ----------------------------------------------------------------------------
//  mod.rs — core module root
// ----------------------------------------------------------------------------
//  Core module root — re-exports coordinator, dispatcher, engine, scanner, worker.
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

//  Core Module: 電脳走査エンジンの中核モジュール
//  Central orchestration module for the Oxide scan engine.
//  Sub-modules:
//   engine       — メインループ / scan lifecycle & error handling
//   scanner      — トレイト契約 / base Scanner trait & contract
//   worker       — スレッドプール / concurrent load-balanced workers
pub mod engine;
pub mod scanner;
pub mod worker;
