// ----------------------------------------------------------------------------
//  progress.rs — scan progress tracker
// ----------------------------------------------------------------------------
//  Thread-safe scan progress tracking with atomic counters for severity
//  buckets (critical, high, medium, low, info), network I/O accounting,
//  ETA calculation, and percentage completion. Used by the display engine.
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
//  進行状況追跡 / scan progress tracking
//  Progress構造体 — アトミックカウンタによるスレッドセーフな進行管理
//                     / thread-safe progress management via atomic counters
//    カウンタグループ / counter groups:
//      基本進行 / base progress — total, current
//      重大度バケット / severity buckets — critical, high, medium, low, info
//      電脳網 / network I/O — bytes_tx, bytes_rx, requests, errors
//      時間計測 / timing — start_time
//
//  インクリメントメソッド / increment methods (書き込み / writes):
//    increment() — 現在の処理数を進める / advances current count
//    add_critical/high/medium/low/info() — 各重大度カウンタを進める / advances severity buckets
//    add_request() — リクエスト数を進める / advances request count
//
//  読み取りメソッド / read methods:
//    get_current/total/vulns/critical/high/medium/low/info_count/errors/requests()
//     — 各カウンタの現在値を取得 / reads current counter values
//    get_bytes_tx/rx() — 転送バイト数を取得 / reads byte transfer counts
//    get_elapsed() — 開始からの経過時間を取得 / reads elapsed time
//    get_percent() — 完了率(0–100%)を計算 / calculates completion percentage
//    is_complete() — 全処理が完了したか判定 / checks if all work is done
//    get_elapsed_string() — 経過時間を"MM:SS"形式で取得 / elapsed as "MM:SS"
//
//  Clone実装 / Clone implementation
//    各アトミックカウンタを現在値で新しいAtomicにコピー / copies each atomic with current value
//    start_timeはそのまま参照 / start_time is copied directly
//
use std::sync::atomic::
        {AtomicU64, AtomicUsize,
                         Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Thread-safe scan progress tracker with per-severity vuln counters,
/// bytes-transferred accounting, and ETA calculation.
pub struct Progress {
    pub total: usize,
    current:   AtomicUsize,
    // Severity buckets
    critical:  AtomicUsize,
    high:      AtomicUsize,
    medium:    AtomicUsize,
    low:       AtomicUsize,
    info:      AtomicUsize,
    // Network accounting
    bytes_tx:  AtomicU64,
    bytes_rx:  AtomicU64,
    requests:  AtomicUsize,
    errors:    AtomicUsize,
    start_time: Instant,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current:    AtomicUsize::new(0),
            critical:   AtomicUsize::new(0),
            high:       AtomicUsize::new(0),
            medium:     AtomicUsize::new(0),
            low:        AtomicUsize::new(0),
            info:       AtomicUsize::new(0),
            bytes_tx:   AtomicU64::new(0),
            bytes_rx:   AtomicU64::new(0),
            requests:   AtomicUsize::new(0),
            errors:     AtomicUsize::new(0),
            start_time: Instant::now(),
        }
    }

    //  Counters 

    pub fn increment(&self) { self.current.fetch_add(1, Ordering::Relaxed); }

    pub fn add_critical(&self) { self.critical.fetch_add(1, Ordering::Relaxed); }
    pub fn add_high(&self)     { self.high.fetch_add(1, Ordering::Relaxed); }
    pub fn add_medium(&self)   { self.medium.fetch_add(1, Ordering::Relaxed); }
    pub fn add_low(&self)      { self.low.fetch_add(1, Ordering::Relaxed); }
    pub fn add_info(&self)     { self.info.fetch_add(1, Ordering::Relaxed); }
    pub fn add_request(&self)  { self.requests.fetch_add(1, Ordering::Relaxed); }

    //  Reads 

    pub fn get_current(&self)  -> usize { self.current.load(Ordering::Relaxed) }
    pub fn get_total(&self)    -> usize { self.total }
    pub fn get_vulns(&self)    -> usize { self.get_critical() + self.get_high() + self.get_medium() + self.get_low() }
    pub fn get_critical(&self) -> usize { self.critical.load(Ordering::Relaxed) }
    pub fn get_high(&self)     -> usize { self.high.load(Ordering::Relaxed) }
    pub fn get_medium(&self)   -> usize { self.medium.load(Ordering::Relaxed) }
    pub fn get_low(&self)      -> usize { self.low.load(Ordering::Relaxed) }
    pub fn get_info_count(&self) -> usize { self.info.load(Ordering::Relaxed) }
    pub fn get_errors(&self)   -> usize { self.errors.load(Ordering::Relaxed) }
    pub fn get_requests(&self) -> usize { self.requests.load(Ordering::Relaxed) }
    pub fn get_bytes_tx(&self) -> u64   { self.bytes_tx.load(Ordering::Relaxed) }
    pub fn get_bytes_rx(&self) -> u64   { self.bytes_rx.load(Ordering::Relaxed) }
    pub fn get_elapsed(&self)  -> Duration { self.start_time.elapsed() }

    pub fn get_percent(&self) -> usize {
        if self.total == 0 { return 0; }
        ((self.get_current() * 100) / self.total).min(100)
    }

    pub fn is_complete(&self) -> bool { self.get_current() >= self.total }

    pub fn get_elapsed_string(&self) -> String {
        let s = self.get_elapsed().as_secs();
        format!("{:02}:{:02}", s / 60, s % 60)
    }

    pub fn clone_arc(self) -> Arc<Self> { Arc::new(self) }
}

impl Clone for Progress {
    fn clone(&self) -> Self {
        Self {
            total:      self.total,
            current:    AtomicUsize::new(self.get_current()),
            critical:   AtomicUsize::new(self.get_critical()),
            high:       AtomicUsize::new(self.get_high()),
            medium:     AtomicUsize::new(self.get_medium()),
            low:        AtomicUsize::new(self.get_low()),
            info:       AtomicUsize::new(self.get_info_count()),
            bytes_tx:   AtomicU64::new(self.get_bytes_tx()),
            bytes_rx:   AtomicU64::new(self.get_bytes_rx()),
            requests:   AtomicUsize::new(self.get_requests()),
            errors:     AtomicUsize::new(self.get_errors()),
            start_time: self.start_time,
        }
    }
}

impl Progress {
    /// Record a failed request (network error, timeout, 5xx handling upstream).
    pub fn add_error(&self) { self.errors.fetch_add(1, Ordering::Relaxed); }
}

// ============================================================================
//  LiveProgress — 電脳ライブ進行表示 / cyberpunk live progress renderer
// ============================================================================
//  Single-line in-place progress display for isolated module scans.
//  Reusable by any scanner:
//      let live = LiveProgress::start("SQLi", urls.len());   // determinate
//      let live = LiveProgress::start("TLS", 0);             // pulse mode
//      ... work, then live.progress().increment() / add_high() ...
//      live.stop().await;
//
//  Determinate (total > 0):
//    ⌁ SQLi   ▕████████████░░░░░░░░░░░░▏ 42/100 ⚑3 18.5r/s 経過00:12 ETA00:07
//  Indeterminate (total == 0): sweeping pulse segment + elapsed + findings.
//
//  Renders at ~10Hz on one redrawn line; stop() paints the final frame and
//  releases the line so post-scan output never tears.
// ============================================================================

use std::io::Write as IoWrite;
use std::sync::atomic::AtomicBool;
use super::display::{FUJI, GIN, HISUI, SAKURA, SHU, TSUYUKUSA, WAKABA};

const BAR_W: usize = 26;

fn ltc(s: &str, c: (u8, u8, u8)) -> String {
    format!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.0, c.1, c.2, s)
}

/// Two-segment colour interpolation HISUI → TSUYUKUSA → FUJI across t ∈ [0,1].
fn bar_gradient(t: f64) -> (u8, u8, u8) {
    let (a, b, c) = (HISUI, TSUYUKUSA, FUJI);
    let (r, g, bl) = if t < 0.5 {
        let lt = t / 0.5;
        (
            a.0 as f64 * (1.0 - lt) + b.0 as f64 * lt,
            a.1 as f64 * (1.0 - lt) + b.1 as f64 * lt,
            a.2 as f64 * (1.0 - lt) + b.2 as f64 * lt,
        )
    } else {
        let lt = (t - 0.5) / 0.5;
        (
            b.0 as f64 * (1.0 - lt) + c.0 as f64 * lt,
            b.1 as f64 * (1.0 - lt) + c.1 as f64 * lt,
            b.2 as f64 * (1.0 - lt) + c.2 as f64 * lt,
        )
    };
    (r as u8, g as u8, bl as u8)
}

pub struct LiveProgress {
    progress: Arc<Progress>,
    stop_flag: Arc<AtomicBool>,
}

impl LiveProgress {
    /// Start the live renderer. `total == 0` switches to indeterminate pulse mode.
    pub fn start(label: &str, total: usize) -> Arc<Self> {
        let lp = Arc::new(Self {
            progress: Arc::new(Progress::new(total)),
            stop_flag: Arc::new(AtomicBool::new(false)),
        });
        let p = lp.progress.clone();
        let s = lp.stop_flag.clone();
        let label = label.to_string();
        let indeterminate = total == 0;
        tokio::spawn(async move {
            Self::render_loop(p, s, label, indeterminate).await;
        });
        lp
    }

    /// Shared counters — increment(), add_high(), add_request(), add_error()…
    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    /// Advance completed-work counter by one unit.
    pub fn tick(&self) {
        self.progress.increment();
    }

    /// Stop rendering, paint the final frame, release the line.
    pub async fn stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        // Renderer polls at 100ms — give it one cycle to draw the final frame.
        tokio::time::sleep(Duration::from_millis(140)).await;
    }

    async fn render_loop(progress: Arc<Progress>, stop: Arc<AtomicBool>, label: String, indeterminate: bool) {
        let mut frame: usize = 0;
        loop {
            if stop.load(Ordering::Relaxed) { break; }
            Self::draw(&progress, &label, indeterminate, frame);
            frame += 1;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        // Final frame + newline so subsequent prints start clean.
        Self::draw(&progress, &label, false, frame);
        println!();
        let _ = std::io::stdout().flush();
    }

    fn draw(p: &Progress, label: &str, indeterminate: bool, frame: usize) {
        let cur = p.get_current();
        let total = p.get_total();
        let vulns = p.get_vulns();
        let errors = p.get_errors();
        let elapsed = p.get_elapsed();
        let secs = elapsed.as_secs_f64().max(0.001);
        let rate = cur as f64 / secs;

        let mut line = String::from("\r\x1B[2K");
        line.push_str(&ltc("⌁", FUJI));
        line.push(' ');
        line.push_str(&ltc(&format!("{:<7}", label), TSUYUKUSA));
        line.push_str(&ltc("▕", GIN));

        if indeterminate || total == 0 {
            // Pulse: bright segment sweeping back and forth over dim track.
            let seg = 8usize.min(BAR_W);
            let span = BAR_W.saturating_sub(seg);
            let raw = frame % (2 * span.max(1));
            let pos = if raw <= span { raw } else { 2 * span.max(1) - raw };
            for i in 0..BAR_W {
                if i >= pos && i < pos + seg {
                    let t = (i - pos) as f64 / seg.max(1) as f64;
                    line.push_str(&ltc("█", bar_gradient(t)));
                } else {
                    line.push_str(&ltc("░", GIN));
                }
            }
            line.push_str(&ltc("▏", GIN));
            line.push_str(&ltc(&format!("経過 {}", fmt_mmss(elapsed.as_secs())), GIN));
        } else {
            let pct = ((cur * 100) / total.max(1)).min(100);
            let filled = (BAR_W * pct) / 100;
            for i in 0..BAR_W {
                if i < filled {
                    let t = i as f64 / BAR_W as f64;
                    line.push_str(&ltc("█", bar_gradient(t)));
                } else {
                    line.push_str(&ltc("░", GIN));
                }
            }
            line.push_str(&ltc("▏", GIN));
            line.push_str(&ltc(&format!("{}/{}", cur, total), HISUI));
            line.push(' ');
            line.push_str(&ltc(&format!("{}%", pct), WAKABA));
            line.push_str(&ltc(" 経過 ", GIN));
            line.push_str(&ltc(&fmt_mmss(elapsed.as_secs()), GIN));
            if rate >= 0.5 {
                let eta = ((total.saturating_sub(cur)) as f64 / rate) as u64;
                line.push_str(&ltc(" ETA ", GIN));
                line.push_str(&ltc(&fmt_mmss(eta), FUJI));
            }
        }

        // Findings flag — vermilion when hot, silver when cold.
        if vulns > 0 {
            line.push_str(&ltc(&format!(" ⚑{}", vulns), SHU));
            let crit = p.get_critical();
            if crit > 0 {
                line.push_str(&ltc(&format!(" ☠{}", crit), SHU));
            }
        } else {
            line.push_str(&ltc(" ⚑0", GIN));
        }
        if errors > 0 {
            line.push_str(&ltc(&format!(" ✗{}", errors), SHU));
        }
        // Sakura sparkle head-tick so the line always feels alive.
        let spark = ["·", "✦", "∗", "✦"][frame % 4];
        line.push_str(&ltc(&format!(" {} ", spark), SAKURA));

        print!("{}", line);
        let _ = std::io::stdout().flush();
    }
}

fn fmt_mmss(secs: u64) -> String {
    format!("{:02}:{:02}", secs / 60, secs % 60)
}
