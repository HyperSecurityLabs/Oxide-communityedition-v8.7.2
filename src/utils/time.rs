// ----------------------------------------------------------------------------
//  time.rs — time utilities
// ----------------------------------------------------------------------------
//  time utilities — duration formatting, timestamp generation, scan time tracking
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

use std::time::{Duration, Instant, SystemTime};
use chrono::{DateTime, Local, Utc};

//  TimeUtil — 時間ユーティリティ / time utility methods
//  All methods are static — provides duration formatting, timestamp generation,
//  sleep helpers, and Unix epoch timestamps for scan timing.
pub struct TimeUtil;

impl TimeUtil {
    //  now() — 現在時刻(ローカル) / current local time
    pub fn now() -> DateTime<Local> {
        Local::now()
    }

    //  now_utc() — 現在時刻(UTC) / current UTC time
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    //  format_timestamp() — タイムスタンプ書式化 / format local time as "YYYY-MM-DD HH:MM:SS"
    pub fn format_timestamp(dt: &DateTime<Local>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    //  format_timestamp_iso() — ISO8601形式 / format UTC as RFC3339
    pub fn format_timestamp_iso(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    //  elapsed_since() — 経過時間 / get duration since start Instant
    pub fn elapsed_since(start: Instant) -> Duration {
        start.elapsed()
    }

    //  format_duration() — 期間書式化 / format Duration as human-readable string
    //   mins > 0: "Xm Ys", otherwise: "X.YYYs"
    pub fn format_duration(duration: Duration) -> String {
        let secs = duration.as_secs();
        let mins = secs / 60;
        let secs = secs % 60;
        let millis = duration.subsec_millis();
        
        if mins > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}.{:03}s", secs, millis)
        }
    }

    //  sleep() — 同期スリープ / blocking sleep
    pub fn sleep(duration: Duration) {
        std::thread::sleep(duration);
    }

    //  sleep_async() — 非同期スリープ / async sleep (tokio)
    pub async fn sleep_async(duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    //  timeout() — タイムアウトラッパー / wrap future with tokio timeout
    pub fn timeout<F, T>(duration: Duration, future: F) -> tokio::time::Timeout<F>
    where
        F: std::future::Future<Output = T>,
    {
        tokio::time::timeout(duration, future)
    }

    //  unix_timestamp() — Unixエポック秒 / current Unix timestamp in seconds
    pub fn unix_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }
}
