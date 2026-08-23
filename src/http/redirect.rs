// ----------------------------------------------------------------------------
//  redirect.rs — HTTP redirect handling
// ----------------------------------------------------------------------------
//  HTTP redirect handling — follows redirect chains, detects open redirects.
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
//  redirect.rs — HTTPリダイレクト処理
//  HTTP redirect handling — follows redirect chains, detects open redirects
//  Locationヘッダー抽出ステータスコードによるリダイレクト分類

use crate::http::response::HttpResponse;

//  extract_redirect_url — Locationヘッダーからリダイレクト先URLを抽出
//  Extracts redirect URL from Location header
pub fn extract_redirect_url(response: &HttpResponse) -> Option<String> {
    response
        .headers
        .get("location")
        .cloned()
}

//  is_303_redirect — 303 See Other リダイレクトを電脳検出
//  Detects 303 See Other redirect (POSTGET変換)
pub fn is_303_redirect(response: &HttpResponse) -> bool {
    response.status == 303
}

//  is_redirect — ステータスコードがリダイレクトか判定
//  Checks if status code is a redirect (301/302/303/307/308)
pub fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 303 | 307 | 308)
}

//  should_switch_to_get — 303時はPOSTGETに切り替え
//  Should switch from POST to GET for 303 redirect
pub fn should_switch_to_get(status: u16) -> bool {
    matches!(status, 303)
}
