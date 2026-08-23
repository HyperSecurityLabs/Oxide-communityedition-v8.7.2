// ----------------------------------------------------------------------------
//  display.rs — terminal display engine
// ----------------------------------------------------------------------------
//  Color palette constants (Gruvbox Dark + Rosé Pine), braille-frame worker
//  slot rendering, animated scan bars, and the ScanBoard/AgentBar real-time
//  terminal UI. Provides Output struct for formatted scan findings display.
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
//  和色パレット定義 / Japanese washoku colour palette
//  Pure traditional Japanese colours — 日本の伝統色
//    SHU     朱 / Vermilion     #E83929  — critical / 危険
//    SAKURA  桜 / Cherry Blossm  #FEDFE1  — pearl pink / 薄
//    HISUI   翡翠 / Jade        #38B48B  — success / 成功
//    WAKABA  若葉 / Fresh Green #8BB85C  — low / 低
//    TSUYUKUSA露草 / Dayflwr Blu#2EA9DF  — info / 情報
//    FUJI    藤 / Wisteria      #8B81C3  — lavender / ラベンダー
//    GIN     銀 / Silver        #91989F  — dim gray / 灰
//    SHIKKOKU漆黒 / Jet Black   #1A1A1A  — background / 背景
//
//  ブレイルスピナー定義 / braille spinner frame definitions
//    BRAILLE, BRAILLE_CW, BRAILLE_CCW — 10フレームの点字アニメーション / 10-frame braille cycles
//    WORKER_OFFSETS — ワーカーごとに開始位置をずらして独立アニメーション / per-worker stagger offset
//
//  WorkerSlot / 個別ワーカースロットの状態管理
//    SlotState — Idle / Active / Done / Error
//    tick() — アトミックカウンタを進めてブレイルフレームを返す / atomically advance braille frame
//    render_row() — ワーカー行をANSIエスケープコードで描画 / renders worker row with ANSI codes
//    truncate_url() — URLを指定長に切り詰める / truncates URL for display
//
//  ScanBoard / メイン電脳走査ボード (リアルタイムUI)
//    フィールド / fields — slots, start, counters, spin_idx, lines_on_screen, light_pos, active
//    メソッドフロー / method flow:
//      new() — Arc<Self>を生成ワーカースロットを初期化 / creates Arc<Self>, initialises slots
//      worker_start() — 指定ワーカーをActive状態に電脳設定 / sets worker to Active
//      worker_done() — ワーカーをDone状態に電脳設定 / marks worker Done
//      worker_error() — ワーカーをError状態に電脳設定 + エラーカウント / marks worker Error + increments
//      render_block() — 全スロットヘッダーを描画 / renders header + all worker rows
//        ヘッダー: ブレイル + ブランド + プログレスバー + カウンタ + タイマー
//        進行バー: render_scan_bar() でパルスアニメーション / pulse animation via render_scan_bar()
//      print_finding_live() — 発見項目をボード上にインライン表示 / prints finding above board
//      render_height() / render() — 補助アクセサ / convenience accessors
//
//  AgentBar / エージェント電脳走査用UI (ScanBoardと同様の構造)
//    フィールド / fields — ScanBoardと同一パターン / same pattern as ScanBoard
//    メソッドフロー / method flow:
//      new() — エージェント数に応じてスロット生成 / creates slots for agent count
//      agent_start_with_phase() — エージェント起動 / starts agent with phase+URL
//      agent_done() — エージェント完了 / marks agent done
//      agent_error() — エージェントエラー / marks agent error
//      render_block() — デュアルブレイル + AGENTSヘッダー / dual-spinner + AGENTS header
//      draw_initial() — 初回描画 / initial draw
//      redraw() — カーソルを戻して再描画 / cursor-up + redraw
//      print_finding() — 発見をボード上に表示 / prints finding above
//      finish() — 最終サマリーを出力 / prints final summary
//
//  sev_badge() / 重大度バッジの整形
//    CRITICAL / HIGH / MEDIUM / LOW / INFO  [ラベル] 形式で色分け / colour-coded badge
//
//  Output / 静的出力ヘルパー
//    print_header() — 区切り線 + タイトル / rule + title
//    print_section() — 下線付きセクションタイトル / underlined section title
//    print_line() — 区切り線 / divider rule
//    print_progress() — 進捗バー (30セグメント) / 30-segment progress bar
//    print_finding_stylish() — 発見項目の詳細書式出力 / stylish finding detail
//    print_scan_complete() — 電脳走査完了サマリー / scan complete summary with severity counts
//
use std::sync::atomic::{AtomicUsize, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use colored::Colorize;
use tokio::sync::RwLock;
use crate::cli::colors::Colors;
use std::sync::atomic::AtomicBool;

const BORDER_WIDTH: usize = 78;

//  Pure Japanese Washoku Palette / 和色
//  Banner palette: 翡翠(HISUI) → 若葉(WAKABA) → 露草(TSUYUKUSA)
//  Accents: 朱(SHU) red · 桜(SAKURA) pearl pink · 藤(FUJI) lavender · 銀(GIN) dim
pub const SHU:        (u8, u8, u8) = (232, 57,  41);   // 朱 / Vermilion
pub const SAKURA:     (u8, u8, u8) = (254, 223, 225);   // 桜 / Cherry blossom
pub const HISUI:      (u8, u8, u8) = (56,  180, 139);   // 翡翠 / Jade
pub const WAKABA:     (u8, u8, u8) = (139, 184, 92);   // 若葉 / Fresh green
pub const TSUYUKUSA:  (u8, u8, u8) = (46,  169, 223);   // 露草 / Dayflower blue
pub const FUJI:       (u8, u8, u8) = (139, 129, 195);   // 藤 / Wisteria
pub const GIN:        (u8, u8, u8) = (145, 152, 159);   // 銀 / Silver
pub const SHIKKOKU:   (u8, u8, u8) = (26,  26,  26);    // 漆黒 / Jet black

// Backward-compat aliases — removed colours mapped to nearest kept colour
pub const SHU_B:      (u8, u8, u8) = SHU;
pub const KURENAI:    (u8, u8, u8) = SHU;
pub const KIN:        (u8, u8, u8) = FUJI;
pub const KIN_B:      (u8, u8, u8) = FUJI;
pub const YAMABUKI:   (u8, u8, u8) = HISUI;
pub const HISUI_B:    (u8, u8, u8) = HISUI;
pub const TSUYUKUSA_B:(u8, u8, u8) = TSUYUKUSA;
pub const AI:         (u8, u8, u8) = TSUYUKUSA;
pub const AI_B:       (u8, u8, u8) = TSUYUKUSA;
pub const KIKYO:      (u8, u8, u8) = FUJI;
pub const KIKYO_B:    (u8, u8, u8) = FUJI;
pub const SUMIRE:     (u8, u8, u8) = FUJI;
pub const AKAGANE:    (u8, u8, u8) = SHU;

pub const ELITE_AQUA:      (u8, u8, u8) = TSUYUKUSA;
pub const ELITE_AQUA_B:    (u8, u8, u8) = TSUYUKUSA;
pub const ELITE_CYAN:      (u8, u8, u8) = TSUYUKUSA;
pub const ELITE_CYAN_B:    (u8, u8, u8) = TSUYUKUSA;
pub const ELITE_BLUE:      (u8, u8, u8) = FUJI;
pub const ELITE_BLUE_B:    (u8, u8, u8) = FUJI;
pub const ELITE_LAVENDER:  (u8, u8, u8) = FUJI;
pub const ELITE_LAVENDER_B:(u8, u8, u8) = FUJI;
pub const ELITE_JADE:      (u8, u8, u8) = HISUI;
pub const ELITE_JADE_B:    (u8, u8, u8) = HISUI;
pub const ELITE_KALI:      (u8, u8, u8) = TSUYUKUSA;

// Legacy name compat
pub const OSAKA_JADE:      (u8, u8, u8) = HISUI;
pub const OSAKA_JADE_B:    (u8, u8, u8) = HISUI;
pub const LAVENDER:        (u8, u8, u8) = FUJI;
pub const LAVENDER_B:      (u8, u8, u8) = FUJI;
pub const LAVENDER_BLUE:   (u8, u8, u8) = TSUYUKUSA;
pub const LAVENDER_BLUE_B: (u8, u8, u8) = TSUYUKUSA;
pub const LIGHT_CYAN:      (u8, u8, u8) = FUJI;
pub const LIGHT_CYAN_B:    (u8, u8, u8) = FUJI;

// Semantic aliases
pub const COL_CRIT: (u8, u8, u8) = SHU;        // 朱 — critical/errors
pub const COL_HIGH: (u8, u8, u8) = SHU;    // 紅 — high severity
pub const COL_MED:  (u8, u8, u8) = HISUI;   // 山吹 — medium
pub const COL_LOW:  (u8, u8, u8) = WAKABA;     // 若葉 — low
pub const COL_INFO: (u8, u8, u8) = TSUYUKUSA;  // 露草 — info
pub const COL_LABEL:(u8, u8, u8) = FUJI;       // 藤 — labels
pub const COL_DIM:  (u8, u8, u8) = GIN;        // 銀 — dim gray

const MAX_WORKERS:  usize = 8;

// Ten-frame braille cycles
const BRAILLE:  &[&str] = &["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
const BRAILLE_CW: &[&str] = &["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"];
const BRAILLE_CCW: &[&str] = &["⠏","⠇","⠧","⠦","⠴","⠼","⠸","⠹","⠙","⠋"];
// Each worker gets a different starting offset so they animate independently
const WORKER_OFFSETS: &[usize] = &[0, 3, 6, 1, 4, 7, 2, 5];

//  WorkerSlot 

#[derive(Clone, Debug, PartialEq)]
pub enum SlotState { Idle, Active, Done, Error }

struct WorkerSlot {
    state:     SlotState,
    phase:     String,          // "scan" | "SQLi" | "XSS" | "LFI" | …
    target:    String,          // current URL (truncated on render)
    spin:      AtomicU8,        // independent braille frame index
    elapsed:   Option<Instant>,
}

impl WorkerSlot {
    fn new(id: usize) -> Self {
        Self {
            state:   SlotState::Idle,
            phase:   String::new(),
            target:  String::new(),
            spin:    AtomicU8::new((WORKER_OFFSETS[id % WORKER_OFFSETS.len()]) as u8),
            elapsed: None,
        }
    }

    /// Advance and return this worker's braille frame.
    fn tick(&self) -> &'static str {
        let idx = self.spin.fetch_add(1, Ordering::Relaxed) as usize;
        BRAILLE[idx % BRAILLE.len()]
    }

    fn elapsed_str(&self) -> String {
        match self.elapsed {
            Some(t) => {
                let s = t.elapsed().as_secs();
                format!("{}:{:02}", s / 60, s % 60)
            }
            None => String::new(),
        }
    }

    fn render_row(&self, id: usize, is_last: bool) -> String {
        let prefix = if is_last { "└" } else { "├" };
        let base = format!("\x1B[2K  {} A{:<2}\x1B[0m",
            prefix.truecolor(HISUI.0, HISUI.1, HISUI.2), id);
        match self.state {
            SlotState::Idle => {
                format!(
                    "{} \x1B[38;2;145;152;159m·\x1B[0m  \x1B[38;2;145;152;159m{:<6}\x1B[0m  \x1B[38;2;145;152;159m{:─<44}\x1B[0m  \x1B[38;2;145;152;159m{:>4}\x1B[0m",
                    base, "idle", "", "--:--",
                )
            }
            SlotState::Active => {
                let spin  = self.tick();
                let phase = if self.phase.len() > 6 { &self.phase[..self.phase.floor_char_boundary(6)] } else { &self.phase };
                let tgt   = truncate_url(&self.target, 44);
                let ela   = self.elapsed_str();
                format!(
                    "{} \x1B[38;2;80;215;170m{}\x1B[0m  \x1B[38;2;139;129;195m{:<6}\x1B[0m  \x1B[38;2;145;152;159m{}\x1B[0m  \x1B[38;2;145;152;159m{:>4}\x1B[0m",
                    base, spin, phase, tgt, ela,
                )
            }
            SlotState::Done => {
                format!(
                    "{} \x1B[38;2;80;215;170m+\x1B[0m  \x1B[38;2;80;215;170m{:<6}\x1B[0m  \x1B[38;2;145;152;159m{:─<44}\x1B[0m  \x1B[38;2;145;152;159m{:>4}\x1B[0m",
                    base, "done", "", self.elapsed_str(),
                )
            }
            SlotState::Error => {
                format!(
                    "{} \x1B[38;2;232;57;41m-\x1B[0m  \x1B[38;2;232;57;41m{:<6}\x1B[0m  \x1B[38;2;145;152;159m{}\x1B[0m  \x1B[38;2;145;152;159m{:>4}\x1B[0m",
                    base, "err", truncate_url(&self.phase, 44), self.elapsed_str(),
                )
            }
        }
    }
}

fn truncate_url(url: &str, max: usize) -> String {
    if url.len() <= max { return url.to_string(); }
    let end = url.floor_char_boundary(max.saturating_sub(1));
    format!("{}…", &url[..end])
}

//  Animated scan bar 
// When active, renders a moving light-green pulse across the bar.
// Normal mode: static filled/unfilled segments.

fn render_scan_bar(_pct: usize, filled: usize, light_pos: &AtomicUsize, active: bool) -> String {
    if !active || filled == 0 {
        let empty = 20usize.saturating_sub(filled);
        return format!("\x1B[38;2;0;200;150m{}\x1B[38;2;60;70;90m{}\x1B[0m",
            "▰".repeat(filled), "▱".repeat(empty));
    }
    let pos = light_pos.fetch_add(1, Ordering::Relaxed) % (filled * 2 - 2).max(1);
    let light = if pos < filled { pos } else { (filled * 2 - 2) - pos };
    let mut out = String::with_capacity(20);
    for i in 0..20 {
        if i < filled {
            if i == light {
                out.push_str("\x1B[38;2;80;215;170;1m█\x1B[0m");
            } else if i.abs_diff(light) <= 1 {
                out.push_str("\x1B[38;2;56;180;139m▓\x1B[0m");
            } else if i.abs_diff(light) <= 2 {
                out.push_str("\x1B[38;2;56;180;139m▒\x1B[0m");
            } else {
                out.push_str("\x1B[38;2;56;180;139m░\x1B[0m");
            }
        } else {
            out.push_str("\x1B[38;2;145;152;159m─\x1B[0m");
        }
    }
    out
}

//  ScanBoard 

pub struct ScanBoard {
    slots:           RwLock<Vec<WorkerSlot>>,
    start:           Instant,
    duration_limit:  RwLock<Option<Duration>>,
    total:           AtomicUsize,
    pub(crate) done: AtomicUsize,
    findings:        AtomicUsize,
    errors:          AtomicUsize,
    spin_idx:        AtomicUsize,
    lines_on_screen: AtomicUsize,
    light_pos:       AtomicUsize,
    active:          AtomicBool,
}

impl ScanBoard {
    pub fn new(worker_count: usize) -> Arc<Self> {
        let n = worker_count.min(MAX_WORKERS);
        let slots = (0..n).map(WorkerSlot::new).collect();
        Arc::new(Self {
            slots:           RwLock::new(slots),
            start:           Instant::now(),
            duration_limit:  RwLock::new(None),
            total:           AtomicUsize::new(0),
            done:            AtomicUsize::new(0),
            findings:        AtomicUsize::new(0),
            errors:          AtomicUsize::new(0),
            spin_idx:        AtomicUsize::new(0),
            lines_on_screen: AtomicUsize::new(0),
            light_pos:       AtomicUsize::new(0),
            active:          AtomicBool::new(false),
        })
    }

    pub fn set_total(&self, n: usize) { self.total.store(n, Ordering::Relaxed); }

    /// Public completion counter bump (bin-side scanners can't touch pub(crate) fields).
    pub fn add_done(&self, n: usize) { self.done.fetch_add(n, Ordering::Relaxed); }

    pub async fn set_duration_limit(&self, secs: u64) {
        if secs > 0 {
            *self.duration_limit.write().await = Some(Duration::from_secs(secs));
        }
    }

    pub async fn worker_start(&self, id: usize, phase: &str, target: &str) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) {
            s.state   = SlotState::Active;
            s.phase   = phase.to_string();
            s.target  = target.to_string();
            if s.elapsed.is_none() { s.elapsed = Some(Instant::now()); }
        }
    }

    pub async fn worker_done(&self, id: usize, found: usize) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) { s.state = SlotState::Done; }
        self.findings.fetch_add(found, Ordering::Relaxed);
    }

    pub async fn worker_error(&self, id: usize, msg: String) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) {
            s.state = SlotState::Error;
            let end = msg.floor_char_boundary(msg.len().min(20));
            s.phase = msg[..end].to_string();
        }
        self.errors.fetch_add(1, Ordering::Relaxed);
        self.done.fetch_add(1, Ordering::Relaxed);
    }

    //  Rendering 

    /// Build the full block string. All N worker slots are always rendered so the
    /// block height is deterministic (1 header + N rows). This guarantees that
    /// `redraw()` can always move the cursor up by the exact same number of lines.
    pub async fn render_block(&self) -> String {
        let slots    = self.slots.read().await;
        let elapsed  = self.start.elapsed();
        let done     = self.done.load(Ordering::Relaxed);
        let total    = self.total.load(Ordering::Relaxed);
        let findings = self.findings.load(Ordering::Relaxed);
        let errors   = self.errors.load(Ordering::Relaxed);
        let mins     = elapsed.as_secs() / 60;
        let secs     = elapsed.as_secs() % 60;

        // Remaining duration countdown
        let remain = self.duration_limit.read().await;
        let timer_str = if let Some(limit) = *remain {
            if let Some(rem) = limit.checked_sub(elapsed) {
                let rm = rem.as_secs() / 60;
                let rs = rem.as_secs() % 60;
                format!("\x1B[38;2;255;177;27m-{:02}:{:02}\x1B[0m \x1B[38;2;145;152;159m{:02}:{:02}\x1B[0m", rm, rs, mins, secs)
            } else {
                format!("\x1B[38;2;232;57;41m\u{2717} TIMEOUT\x1B[0m \x1B[38;2;145;152;159m{:02}:{:02}\x1B[0m", mins, secs)
            }
        } else {
            format!("\x1B[38;2;145;152;159m{:02}:{:02}\x1B[0m", mins, secs)
        };

        // Header spinner
        let spin = {
            let idx = self.spin_idx.fetch_add(1, Ordering::Relaxed);
            BRAILLE[idx % BRAILLE.len()]
        };

        let pct    = if total > 0 { ((done * 100) / total).min(100) } else { 0 };
        let filled = ((pct * 20) / 100).min(20);
        let bar    = render_scan_bar(pct, filled, &self.light_pos, self.active.load(Ordering::Relaxed));

        let vuln_s = if findings > 0 {
            format!("\x1B[38;2;255;50;50;1m{} det\x1B[0m", findings)
        } else {
            format!("\x1B[38;2;145;152;159m0 det\x1B[0m",)
        };
        let err_s = if errors > 0 {
            format!("\x1B[38;2;255;180;50m{} err\x1B[0m", errors)
        } else {
            format!("\x1B[38;2;145;152;159m0 err\x1B[0m")
        };

        // Header line (always present) — braille inside [ ] with 翡翠
        let spin_bracketed = format!("\x1B[38;2;{};{};{}m[{}]\x1B[0m",
            HISUI.0, HISUI.1, HISUI.2, spin);
        let header = format!(
            "\x1B[2K{} {}  \x1B[38;2;80;215;170m{}\x1B[0m  \
             \x1B[97m{}/{}\x1B[0m \x1B[38;2;46;169;223m({}%)\x1B[0m  \
             {}  {}  {}",
            spin_bracketed,
            Colors::brand("OXIDE"),
            bar,
            done, total, pct,
            vuln_s, err_s,
            timer_str,
        );

        // Always render ALL slots — even idle are shown as dim placeholders.
        // This guarantees block height = 1 + worker_count, always the same.
        let row_count = slots.len();
        let mut out = header;
        let last = row_count.saturating_sub(1);
        for (id, slot) in slots.iter().enumerate() {
            out.push('\n');
            out.push_str(&slot.render_row(id, id == last));
        }

        self.lines_on_screen.store(1 + row_count, Ordering::Relaxed);
        out
    }

    pub async fn print_finding_live(&self, severity: &str, title: &str, url: &str) {
        let sev   = sev_badge(severity);
        let url_s = truncate_url(url, 55);
        let prev  = self.lines_on_screen.load(Ordering::Relaxed);

        if prev > 0 { print!("\x1B[{}A\x1B[0G", prev); }
        print!("\x1B[2K");
        println!("  {} \x1B[38;2;139;129;195m{}\x1B[0m  \x1B[38;2;46;169;223m{}\x1B[0m", sev, title, url_s);

        let block = self.render_block().await;
        println!("{}", block);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }

    pub async fn render_height(&self) -> usize {
        self.lines_on_screen.load(Ordering::Relaxed)
    }
    pub async fn render(&self)      -> String { self.render_block().await }
}

//  AgentBar 
//
// Fixed N-line block for AgentPool — same in-place redraw as ScanBoard but
// dedicated to the agent scan phase. Supports up to 8 agents.
//
//    AGENTS  [========------------]  42/100 (42%)  done:38  err:2  01:23
//     A0  scan   https://target.com/admin          0:04
//     A1  scan   https://target.com/login          0:02

pub struct AgentBar {
    slots:           RwLock<Vec<WorkerSlot>>,
    start:           Instant,
    total:           AtomicUsize,
    done:            AtomicUsize,
    errors:          AtomicUsize,
    findings:        AtomicUsize,
    spin_idx:        AtomicUsize,
    lines_on_screen: AtomicUsize,
    light_pos:       AtomicUsize,
    active:          AtomicBool,
}

impl AgentBar {
    pub fn new(agent_count: usize) -> Arc<Self> {
        let n = agent_count.min(MAX_WORKERS);
        let slots = (0..n).map(WorkerSlot::new).collect();
        Arc::new(Self {
            slots:           RwLock::new(slots),
            start:           Instant::now(),
            total:           AtomicUsize::new(0),
            done:            AtomicUsize::new(0),
            errors:          AtomicUsize::new(0),
            findings:        AtomicUsize::new(0),
            spin_idx:        AtomicUsize::new(0),
            lines_on_screen: AtomicUsize::new(0),
            light_pos:       AtomicUsize::new(0),
            active:          AtomicBool::new(false),
        })
    }

    pub fn set_active(&self) { self.active.store(true, Ordering::Relaxed); }

    pub fn set_total(&self, n: usize) { self.total.store(n, Ordering::Relaxed); }

    pub async fn agent_start_with_phase(&self, id: usize, phase: &str, url: &str) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) {
            s.state  = SlotState::Active;
            s.phase  = phase.to_string();
            s.target = url.to_string();
            if s.elapsed.is_none() { s.elapsed = Some(Instant::now()); }
        }
    }

    pub async fn agent_done(&self, id: usize, found: usize) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) { s.state = SlotState::Done; }
        self.findings.fetch_add(found, Ordering::Relaxed);
    }

    pub async fn agent_error(&self, id: usize) {
        let mut slots = self.slots.write().await;
        if let Some(s) = slots.get_mut(id) { s.state = SlotState::Error; }
        self.errors.fetch_add(1, Ordering::Relaxed);
    }

    pub async fn add_finding(&self) {
        self.findings.fetch_add(1, Ordering::Relaxed);
    }

    pub fn progress_tick(&self) {
        self.done.fetch_add(1, Ordering::Relaxed);
    }

    async fn render_block(&self) -> String {
        let slots    = self.slots.read().await;
        let elapsed  = self.start.elapsed();
        let done     = self.done.load(Ordering::Relaxed);
        let total    = self.total.load(Ordering::Relaxed);
        let errors   = self.errors.load(Ordering::Relaxed);
        let findings = self.findings.load(Ordering::Relaxed);
        let mins     = elapsed.as_secs() / 60;
        let secs     = elapsed.as_secs() % 60;

        let spin_idx = self.spin_idx.fetch_add(1, Ordering::Relaxed);
        let cw = BRAILLE_CW[spin_idx % 10];
        let ccw = BRAILLE_CCW[spin_idx % 10];

        let pct    = if total > 0 { ((done * 100) / total).min(100) } else { 0 };
        let filled = ((pct * 20) / 100).min(20);
        let bar    = render_scan_bar(pct, filled, &self.light_pos, self.active.load(Ordering::Relaxed));

        let vuln_s = if findings > 0 {
            format!("\x1B[38;2;232;57;41;1m{}\x1B[0m", findings)
        } else {
            format!("\x1B[38;2;145;152;159m{}\x1B[0m", findings)
        };

        let dual_spin = format!("\x1B[38;2;{};{};{}m[{} {}]\x1B[0m",
            OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2, cw, ccw);
        let header = format!(
            "\x1B[2K{} \x1B[38;2;56;180;139m[+]\x1B[0m\x1B[97mAGENTS\x1B[0m  [{}]  \
             \x1B[97m{}/{}\x1B[0m \x1B[38;2;145;152;159m({}%)\x1B[0m  \
             \x1B[38;2;232;57;41mVULN:\x1B[0m{}  \x1B[38;2;145;152;159merr:{}\x1B[0m  \x1B[38;2;145;152;159m{:02}:{:02}\x1B[0m",
            dual_spin, bar, done, total, pct, vuln_s, errors, mins, secs,
        );

        // Always render ALL slots for deterministic height
        let row_count = slots.len();
        let mut out = header;
        let last = row_count.saturating_sub(1);
        for (id, slot) in slots.iter().enumerate() {
            out.push('\n');
            out.push_str(&slot.render_row(id, id == last));
        }

        self.lines_on_screen.store(1 + row_count, Ordering::Relaxed);
        out
    }

    pub async fn draw_initial(&self) {
        let block = self.render_block().await;
        print!("{}\n", block);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }

    pub async fn redraw(&self) {
        let prev = self.lines_on_screen.load(Ordering::Relaxed);
        if prev > 0 { print!("\x1B[{}A\x1B[0G", prev); }
        let block = self.render_block().await;
        println!("{}", block);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }

    /// Print a finding above the agent block.
    pub async fn print_finding(&self, severity: &str, title: &str, url: &str) {
        let sev   = sev_badge(severity);
        let url_s = truncate_url(url, 55);
        let prev  = self.lines_on_screen.load(Ordering::Relaxed);
        if prev > 0 { print!("\x1B[{}A\x1B[0G", prev); }
        print!("\x1B[2K");
        println!("  {} \x1B[38;2;139;129;195m{}\x1B[0m  \x1B[38;2;46;169;223m{}\x1B[0m", sev, title, url_s);
        let block = self.render_block().await;
        println!("{}", block);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }

    pub fn finish(&self) {
        let elapsed  = self.start.elapsed();
        let done     = self.done.load(Ordering::Relaxed);
        let findings = self.findings.load(Ordering::Relaxed);
        let errors   = self.errors.load(Ordering::Relaxed);
        println!();
        println!(
            "\x1B[38;2;56;180;139m[+]\x1B[0m\x1B[97mAGENTS\x1B[0m  \
             \x1B[97m{} scanned\x1B[0m  \x1B[38;2;232;57;41mVULN:{}\x1B[0m  \
             \x1B[38;2;145;152;159merr:{}  {:02}:{:02}\x1B[0m",
            done, findings, errors,
            elapsed.as_secs() / 60, elapsed.as_secs() % 60,
        );
    }
}

pub fn sev_badge(severity: &str) -> String {
    let (col, label) = match severity.to_uppercase().as_str() {
        "DANGEROUS" => (COL_CRIT, "DANGEROUS"),
        "CRITICAL"  => (COL_CRIT, "CRITICAL"),
        "HIGH"      => (COL_HIGH, "  HIGH  "),
        "MEDIUM"    => (COL_MED,  " MEDIUM "),
        "LOW"       => (COL_LOW,  "  LOW   "),
        _           => (COL_INFO, "  INFO  "),
    };
    format!("\x1B[38;2;{};{};{}m[{}]\x1B[0m", col.0, col.1, col.2, label)
}

//  Output 

pub struct Output;

impl Output {
    pub fn print_header(title: &str) {
        let rule = "─".repeat(BORDER_WIDTH - 8);
        println!("\n{}", rule.truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2));
        println!("  {}", title.truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2).bold());
        println!("{}", rule.truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2));
    }
    pub fn print_section(title: &str) {
        println!("  {}", title.truecolor(LAVENDER_BLUE.0, LAVENDER_BLUE.1, LAVENDER_BLUE.2).underline());
    }
    pub fn print_line() {
        println!("{}", "─".repeat(BORDER_WIDTH).truecolor(LAVENDER_BLUE.0, LAVENDER_BLUE.1, LAVENDER_BLUE.2));
    }
    pub fn print_progress(current: usize, total: usize, vulns: usize, elapsed: &str) {
        let pct    = if total > 0 { (current * 100) / total } else { 0 };
        let filled = (pct * 30) / 100;
        let bar    = format!("{}{}", "─".repeat(filled), "·".repeat(30usize.saturating_sub(filled)));
        println!("  {} {}%  {}  {} vulns  {}",
            format!("[{:.>30}]", bar).truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2),
            format!("{:>3}", pct).truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2),
            format!("{:>4}/{}", current, total).truecolor(LAVENDER_BLUE.0, LAVENDER_BLUE.1, LAVENDER_BLUE.2),
            vulns.to_string().truecolor(COL_CRIT.0, COL_CRIT.1, COL_CRIT.2).bold(),
            elapsed.truecolor(COL_DIM.0, COL_DIM.1, COL_DIM.2));
    }
    pub fn print_finding_stylish(severity: &str, title: &str, url: &str, evidence: &str) {
        let (sev_col, badge) = match severity {
            "Critical" => (COL_CRIT, "[CRITICAL]"),
            "High"     => (COL_HIGH, "[  HIGH  ]"),
            "Medium"   => (COL_MED,  "[ MEDIUM ]"),
            "Low"      => (COL_LOW,  "[  LOW   ]"),
            "Info"     => (COL_INFO, "[  INFO  ]"),
            _          => (COL_DIM,  "[ UNKNOWN]"),
        };
        let badge_s = badge.truecolor(sev_col.0, sev_col.1, sev_col.2).bold().to_string();
        let title_s = title.truecolor(LAVENDER_B.0, LAVENDER_B.1, LAVENDER_B.2).to_string();
        let url_s   = url.truecolor(COL_DIM.0, COL_DIM.1, COL_DIM.2).italic().to_string();

        println!("  {}  {}  // {}", badge_s, title_s, url_s);

        if !evidence.is_empty() && evidence.len() > 6 {
            let ev = if evidence.len() > 120 {
                let end = evidence.floor_char_boundary(120);
                format!("{}…", &evidence[..end])
            } else {
                evidence.to_string()
            };
            println!("      {}", ev.truecolor(COL_DIM.0, COL_DIM.1, COL_DIM.2));
        }
    }
    pub fn print_scan_complete(duration: &str, total_requests: usize, findings: &[crate::detection::analyzer::Finding]) {
        let rule = "─".repeat(BORDER_WIDTH - 8);
        println!("\n{}", rule.truecolor(OSAKA_JADE.0, OSAKA_JADE.1, OSAKA_JADE.2));
        println!("  {}  {}",
            "◆".truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2),
            "SCAN COMPLETE".truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2).bold());
        println!("{}", rule.truecolor(OSAKA_JADE.0, OSAKA_JADE.1, OSAKA_JADE.2));

        let critical = findings.iter().filter(|f| matches!(f.severity, crate::detection::analyzer::Severity::Critical)).count();
        let high     = findings.iter().filter(|f| matches!(f.severity, crate::detection::analyzer::Severity::High)).count();
        let medium   = findings.iter().filter(|f| matches!(f.severity, crate::detection::analyzer::Severity::Medium)).count();
        let low      = findings.iter().filter(|f| matches!(f.severity, crate::detection::analyzer::Severity::Low)).count();
        let info     = findings.iter().filter(|f| matches!(f.severity, crate::detection::analyzer::Severity::Info)).count();

        println!("  {} {}  {} {}  {} {}  {} {}  {} {}",
            "◉".truecolor(COL_CRIT.0, COL_CRIT.1, COL_CRIT.2),
            format!("{:>2} critical", critical).truecolor(COL_CRIT.0, COL_CRIT.1, COL_CRIT.2),
            "◉".truecolor(COL_HIGH.0, COL_HIGH.1, COL_HIGH.2),
            format!("{:>2} high", high).truecolor(COL_HIGH.0, COL_HIGH.1, COL_HIGH.2),
            "◉".truecolor(COL_MED.0, COL_MED.1, COL_MED.2),
            format!("{:>2} medium", medium).truecolor(COL_MED.0, COL_MED.1, COL_MED.2),
            "◉".truecolor(COL_LOW.0, COL_LOW.1, COL_LOW.2),
            format!("{:>2} low", low).truecolor(COL_LOW.0, COL_LOW.1, COL_LOW.2),
            "◉".truecolor(COL_INFO.0, COL_INFO.1, COL_INFO.2),
            format!("{:>2} info", info).truecolor(COL_INFO.0, COL_INFO.1, COL_INFO.2),
        );
        println!("  {} {}  {} {}",
            "·".truecolor(OSAKA_JADE.0, OSAKA_JADE.1, OSAKA_JADE.2),
            format!("Duration: {}", duration).truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2),
            "⚑".truecolor(OSAKA_JADE_B.0, OSAKA_JADE_B.1, OSAKA_JADE_B.2),
            format!("Requests: {}", total_requests).truecolor(LAVENDER.0, LAVENDER.1, LAVENDER.2),
        );
        println!("{}\n", rule.truecolor(OSAKA_JADE.0, OSAKA_JADE.1, OSAKA_JADE.2));
    }
}
