// ----------------------------------------------------------------------------
//  spinner.rs — braille spinner animation
// ----------------------------------------------------------------------------
//  Provides the Spinner struct with independent braille-dot frame sequences
//  for terminal animation. Supports multiple named spinner patterns (CW, CCW,
//  header, vuln, finger) with atomic frame advancement for thread safety.
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
//  点字スピナーアニメーション / braille spinner animation system
//  Spinner構造体 — 独立したフレームシーケンスでアニメーション / independent frame sequences
//    frames: &'static [&'static str] — 点字フレームの静的スライス / static braille frame slice
//    current: AtomicUsize — アトミックフレームインデックス / atomic frame index (thread-safe)
//
//  フレームセット / frame set definitions (静的 / static):
//    FRAMES_CW — 時計回りシーケンス / clockwise sequence
//    FRAMES_CCW — 反時計回りシーケンス / counter-clockwise sequence
//    FRAMES_A/B/C — 追加の位相バリアント / additional phase variants
//
//  名前付きコンストラクタ / named constructors:
//    path_spinner()  FRAMES_CW / パス電脳走査用 / for path scanning
//    param_spinner()  FRAMES_CCW / パラメータ電脳走査用 / for param scanning
//    header_spinner()  FRAMES_A / ヘッダー用 / for header animation
//    vuln_spinner()  FRAMES_B / 脆弱性電脳検出用 / for vulnerability discovery
//    finger_spinner()  FRAMES_C / フィンガープリント用 / for fingerprinting
//
//  操作メソッド / operation methods:
//    next() — フレームを進めて現在の点字文字を返す / advances frame, returns braille char
//      fetch_add + moduloで循環 / fetch_add with modulo for wrapping
//    tick_strings() — フレームセットを動的に変更 / dynamically changes frame set
//
use std::sync::atomic::{AtomicUsize, Ordering};

/// Braille-dot spinner with per-instance phase offset so parallel workers
/// animate independently without synchronizing.
pub struct Spinner {
    frames: &'static [&'static str],
    current: AtomicUsize,
}

// All frame sets are static slices — no heap allocation per spinner.
static FRAMES_CW:  &[&str] = &["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
static FRAMES_CCW: &[&str] = &["⠏","⠇","⠧","⠦","⠴","⠼","⠸","⠹","⠙","⠋"];
static FRAMES_A:   &[&str] = &["⠧","⠦","⠴","⠼","⠸","⠹","⠙","⠋","⠏","⠇"];
static FRAMES_B:   &[&str] = &["⠼","⠴","⠦","⠧","⠇","⠏","⠋","⠙","⠹","⠸"];
static FRAMES_C:   &[&str] = &["⠸","⠹","⠙","⠋","⠏","⠇","⠧","⠦","⠴","⠼"];

impl Clone for Spinner {
    fn clone(&self) -> Self {
        Self {
            frames: self.frames,
            current: AtomicUsize::new(self.current.load(Ordering::Relaxed)),
        }
    }
}

impl Spinner {
    fn new(frames: &'static [&'static str]) -> Self {
        Self { frames, current: AtomicUsize::new(0) }
    }

    //  Named constructors 

    pub fn path_spinner()   -> Self { Self::new(FRAMES_CW) }
    pub fn param_spinner()  -> Self { Self::new(FRAMES_CCW) }
    pub fn header_spinner() -> Self { Self::new(FRAMES_A) }
    pub fn vuln_spinner()   -> Self { Self::new(FRAMES_B) }
    pub fn finger_spinner() -> Self { Self::new(FRAMES_C) }

    //  Advance / read 

    /// Advance and return the next frame.
    pub fn next(&self) -> &'static str {
        let idx = self.current.fetch_add(1, Ordering::Relaxed) % self.frames.len();
        self.frames[idx]
    }

    /// Set a custom frame set for this spinner.
    pub fn tick_strings(&mut self, frames: &'static [&'static str]) {
        self.frames = frames;
    }

}
