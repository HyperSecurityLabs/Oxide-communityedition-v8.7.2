// ----------------------------------------------------------------------------
//  mod.rs — HTTP module root
// ----------------------------------------------------------------------------
//  HTTP module root — re-exports HTTP client and related types.
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
//  http/ モジュールルート — HTTP通信モジュール
//  HTTP module root — re-exports all HTTP submodules
//  各サブモジュールの役割：
//   client       —  HTTPクライアント構築 (reqwestラッパー)
//   cookies      —  クッキー管理とセッション永続化
//   crawl_types  —  電脳収集用データ構造
//   headless     —  ヘッドレスブラウザレンダリング
//   headers      — ※ セキュリティヘッダー監査
//   redirect     —  リダイレクト処理
//   request      —  HTTPリクエストビルダー
//   response     —  HTTPレスポンスパーサー
//   tls          —  TLSバージョンチェック
//   useragents   —  User-Agentローテーション

pub mod client;
pub mod cookies;
pub mod crawl_types;
pub mod headless;
pub mod headers;
pub mod redirect;
pub mod request;
pub mod response;
pub mod tls;
pub mod useragents;
