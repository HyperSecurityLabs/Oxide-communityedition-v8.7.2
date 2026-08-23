// ----------------------------------------------------------------------------
//  downloader.rs — remote resource downloader
// ----------------------------------------------------------------------------
//  remote resource downloader — fetches payload lists, wordlists, update files
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

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

//  Downloader — リモートリソースダウンローダー / remote resource downloader
//  Downloads payload lists, wordlists, and update files from remote sources.
//   new(): creates target-specific download dir (downloads/{domain}_{timestamp})
//   base_dir(): returns the download directory path for file I/O
pub struct Downloader {
    base_dir: PathBuf,
}

impl Downloader {
    //  new() — ダウンローダー初期化 / downloader setup
    //   Extracts domain from target_url (strips scheme, path, port colon)
    //   Generates Unix timestamp for unique directory naming
    //   Creates download path: downloads/{domain}_{timestamp}
    //   base_dir() returns the path for subsequent file I/O operations
    pub fn new(target_url: &str) -> Self {
        let domain = target_url
            .replace("https://", "")
            .replace("http://", "")
            .split('/')
            .next()
            .unwrap_or("unknown")
            .replace(':', "_");

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let base_dir = PathBuf::from(format!("downloads/{}_{}", domain, ts));
        Self { base_dir }
    }

    pub fn base_dir(&self) -> &Path { &self.base_dir }
}
