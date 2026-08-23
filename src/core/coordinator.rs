// ----------------------------------------------------------------------------
//  coordinator.rs — scan coordinator
// ----------------------------------------------------------------------------
//  Scan coordinator — manages multi-scanner execution flow, aggregates results.
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

use std::sync::atomic::{AtomicUsize, Ordering};

//  Coordinator: マルチスキャナ調整役
//  Orchestrates multiple scanner instances across target surfaces.
//  Flow:
//   1. Distributor assigns task batches  parallel scanners ( AtomicUsize cursor)
//   2. Each scanner executes independently  results stream via mpsc channel
//   3. Results are aggregated  passed to Analyzer for finding extraction
//  マルチスキャナの実行フロー / 結果を集約してAnalyzerに渡す
pub struct Coordinator {
    total_tasks: AtomicUsize,
}

impl Coordinator {
    pub fn new(total: usize) -> Self {
        Self {
            total_tasks: AtomicUsize::new(total),
        }
    }
}

impl Clone for Coordinator {
    fn clone(&self) -> Self {
        Self {
            total_tasks: AtomicUsize::new(self.total_tasks.load(Ordering::SeqCst)),
        }
    }
}
