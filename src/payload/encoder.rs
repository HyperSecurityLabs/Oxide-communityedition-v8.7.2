// ----------------------------------------------------------------------------
//  encoder.rs — payload encoder
// ----------------------------------------------------------------------------
//  Payload encoder — URL, base64, hex encoding for WAF/IDS evasion.
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

//  Encoder: ペイロードエンコーダ
//  Encoding techniques for WAF/IDS evasion.
//  エンコード手法:
//   ➊ url_encode — URLパーセントエンコーディング (%XX)
//   ➋ base64_encode — Base64標準エンコーディング
//   ➌ hex_encode — 16進数エンコーディング
//   ➍ double_encode — 二重URLエンコード (%25XX  %XX)
//  WAFバイパスに有効: 二重エンコードで復号処理を混乱させる
pub struct Encoder;

impl Encoder {
    //  url_encode: URLパーセントエンコード
    //  Percent-encodes special characters for safe URL transmission.
    //  例: "<script>"  "%3Cscript%3E"
    pub fn url_encode(input: &str) -> String {
        urlencoding::encode(input).to_string()
    }

    //  base64_encode: Base64エンコード
    //  Encodes input to standard Base64 (RFC 4648) for data encoding.
    //  例: "alert(1)"  "YWxlcnQoMSk="
    pub fn base64_encode(input: &str) -> String {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.encode(input.as_bytes())
    }

    //  hex_encode: 16進数エンコード
    //  Converts input bytes to hexadecimal string representation.
    //  例: "id"  "6964"
    pub fn hex_encode(input: &str) -> String {
        hex::encode(input.as_bytes())
    }

    //  double_encode: 二重URLエンコード
    //  Applies URL encoding twice — bypasses decoders that decode only once.
    //  例: "<"  "%253C" (first: %3C, second: %253C)
    pub fn double_encode(input: &str) -> String {
        Self::url_encode(&Self::url_encode(input))
    }
}
