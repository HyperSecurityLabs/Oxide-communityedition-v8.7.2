// ----------------------------------------------------------------------------
//  mod.rs — utility module root
// ----------------------------------------------------------------------------
//  utility module root — re-exports downloader, time, URL utilities
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
//  ユーティリティモジュール — Utility Module Root
//   downloader.rs — ダウンローダー / HTTP GET, progress tracking, file save
//   time.rs       — 時間ユーティリティ / duration formatting, sleep, timestamps
//   url.rs        — URL処理 / parsing, normalization, validation, encoding
// ---------------------------------------------------------------------------

pub mod downloader;
pub mod socket;
pub mod time;
pub mod url;
