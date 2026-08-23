// ----------------------------------------------------------------------------
//  colors.rs — color constants and helpers
// ----------------------------------------------------------------------------
//  Defines the Colors struct with ANSI truecolor helper methods for output
//  formatting (warning, ok, brand). Also provides the print_status function
//  for standardized status messages with severity-based coloring.
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
//  カラーパレット定義 / colour palette constants
//  Colors構造体 — ANSI TrueColorヘルパーメソッド / TrueColor helper methods
//    warning() — ラベンダー色の警告テキスト / lavender warning text
//    ok() — ジェード色の成功テキスト / jade success text
//    brand() — 太字ジェードのブランドテキスト / bold jade brand text
//  ステータス表示関数 / status display function
//    print_status() — 重大度に応じた色分けでステータスを出力 / outputs colour-coded status messages
//    "OK"  bright green / 明るい緑
//    "ERROR"  bright red bold / 明るい赤太字
//    "WARN"  bright yellow / 明るい黄
//    "INFO"  bright cyan / 明るいシアン
//    "VULN"  truecolor red bold / 赤太字
//    "CRITICAL"  truecolor red bold / 赤太字
//    _ (default)  汎用フォーマット / generic format
//
use colored::Colorize;

const FUJI: (u8,u8,u8) = (139,129,195);   // 藤 / Wisteria
const HISUI: (u8,u8,u8) = (56,180,139);   // 翡翠 / Jade

pub struct Colors;

impl Colors {
    pub fn warning(text: &str) -> String {
        text.truecolor(FUJI.0, FUJI.1, FUJI.2).to_string()
    }

    pub fn ok(text: &str) -> String {
        text.truecolor(HISUI.0, HISUI.1, HISUI.2).to_string()
    }

    pub fn brand(text: &str) -> String {
        text.truecolor(HISUI.0, HISUI.1, HISUI.2).bold().to_string()
    }
}

pub fn print_status(status: &str, message: &str) {
    match status {
        "OK"       => println!("{} {}", "翡翠".truecolor(56,180,139), message),
        "ERROR"    => println!("{} {}", "朱".truecolor(232,57,41).bold(), message),
        "WARN"     => println!("{} {}", "山吹".truecolor(56,180,139), message),
        "INFO"     => println!("{} {}", "露草".truecolor(46,169,223), message),
        "VULN"     => println!("{} {}", "朱".truecolor(232,57,41).bold(), message),
        "CRITICAL" => println!("{} {}", "朱".truecolor(232,57,41).bold(), message),
        _          => println!("[{:^6}] {}", status, message),
    }
}
