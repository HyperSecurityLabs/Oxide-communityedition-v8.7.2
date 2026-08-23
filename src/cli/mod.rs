// ----------------------------------------------------------------------------
//  mod.rs — CLI module structure
// ----------------------------------------------------------------------------
//  Root module for the CLI subsystem. Re-exports submodules (args, colors,
//  config, display, output, parser, progress, spinner) that handle command-line
//  argument parsing, terminal output, configuration, and progress rendering.
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
//  CLIモジュール構造 / CLI module structure overview
//   args.rs — clapによるコマンドライン電脳引数解析 / argument parsing via clap derive
//   colors.rs — ANSI TrueColor定数とステータス表示 / colour constants and status helpers
//   config.rs — TOML電脳設定の読み書き生成 / config load/save/generate
//   display.rs — ターミナルUIエンジン / terminal display engine (ScanBoard, AgentBar, braille)
//   output.rs — 下位互換性のための再エクスポート / backward-compat re-export shim
//   parser.rs — URL・ヘッダ・クッキー・モジュールパース / input parsing utilities
//   progress.rs — アトミックカウンタによる進行追跡 / atomic scan progress tracker
//   spinner.rs — 点字スピナーアニメーション / braille spinner animation system
//
pub mod args;
pub mod colors;
pub mod config;
pub mod display;
pub mod output;
pub mod parser;
pub mod progress;
pub mod spinner;
