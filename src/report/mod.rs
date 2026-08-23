// ----------------------------------------------------------------------------
//  mod.rs — report module root
// ----------------------------------------------------------------------------
//  Report module root — re-exports all report format generators.
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

//  Report Module: レポート出力モジュール
//  Scan report generation in multiple formats for interoperability.
//  Sub-modules:
//   generator — レポート生成フロー (収集整形出力) / finding collection & format dispatch
//   csv      — カンマ区切り出力 / spreadsheet-compatible CSV export
//   html     — サイバーパンク形式HTML / styled HTML with severity color coding
//   json     — 構造化JSON出力 / programmatic JSON serialization
//   xml      — XML出力 / schema-compliant XML representation
pub mod csv;
pub mod generator;
pub mod html;
pub mod json;
pub mod xml;
