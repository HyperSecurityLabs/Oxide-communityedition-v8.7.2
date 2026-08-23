// ----------------------------------------------------------------------------
//  mod.rs — payload module root
// ----------------------------------------------------------------------------
//  Payload module root — re-exports all payload libraries.
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

//  Payload Module: ペイロード生成ライブラリ
//  Payload generation and manipulation library for vulnerability scanning.
//  Sub-modules:
//   command_injection — Unix/Windows/ブラインドコマンド / OS command payloads
//   encoder           — URL/Base64/Hex/二重エンコード / encoding techniques
//   fuzzer            — ペイロード順列生成 / payload permutation & fuzzing
//   generator         — テクノロジー認識ペイロード / tech-aware payload selection
//   lfi               — パストラバーサル / LFI path traversal & PHP wrappers
//   mutator           — ケース/エンコード/パディング変換 / mutation strategies
//   sql_injection     — エラー/UNION/ブール/時間/NoSQL / SQLi payload types
//   xss               — Reflected/Stored/DOM/CSPバイパス / XSS attack variants
pub mod command_injection;
pub mod encoder;
pub mod fuzzer;
pub mod generator;
pub mod lfi;
pub mod mutator;
pub mod sql_injection;
pub mod xss;
