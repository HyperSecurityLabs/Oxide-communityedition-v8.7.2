// ----------------------------------------------------------------------------
//  mod.rs — Detection module root
// ----------------------------------------------------------------------------
//  Detection module root — re-exports all detection submodules.
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
//  detection/ モジュールルート — サブモジュールの再エクスポート
//  Module root — re-exports all detection submodules
//  各サブモジュールは特定の電脳検出機能を担当：
//   analyzer   —  脆弱性発見構造体 (Finding) と分析パイプライン
//   behavior   —  WAF識別 / エラーページ電脳検出 / 技術スタック分析
//   confirm    —  セカンダリプローブによる偽陽性除去
//   matcher    —  正規表現ベースのシグネチャマッチング
//   scorer     — ※ 編集距離 (Levenshtein) + ベイズ信頼度スコアリング
//   signatures —  シグネチャデータベース (ID・重大度・パターン)
//   timing     —  タイムベース盲検インジェクション用応答時間測定

pub mod analyzer;
pub mod behavior;
pub mod confirm;
pub mod matcher;
pub mod scorer;
pub mod signatures;
pub mod timing;
