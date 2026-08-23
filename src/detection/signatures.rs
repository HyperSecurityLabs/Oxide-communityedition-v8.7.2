// ----------------------------------------------------------------------------
//  signatures.rs — Signature database
// ----------------------------------------------------------------------------
//  Signature database — maps vulnerability patterns to detection signatures.
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
//  signatures.rs — シグネチャデータベース
//  Signature database — maps vulnerability IDs to detection patterns
//  HashMap<String, VulnSignature> で管理IDによる高速検索を実現

use std::collections::HashMap;

//  SignatureDatabase — シグネチャデータベース
//  Centralized signature store — maps IDs to vulnerability signatures
pub struct SignatureDatabase {
    signatures: HashMap<String, VulnSignature>,
}

//  VulnSignature — 脆弱性シグネチャの完全な定義
//  Complete vulnerability signature definition
//  id          — 一意識別子 (例: OXIDE-001)
//  name        — 発見名
//  severity    — 重大度 (文字列)
//  pattern     — 電脳検出パターン (部分一致)
//  description — 説明
//  remediation — 修正方法
#[derive(Clone, Debug)]
pub struct VulnSignature {
    pub id: String,
    pub name: String,
    pub severity: String,
    pub pattern: String,
    pub description: String,
    pub remediation: String,
}

//  シグネチャデータベースの実装
//  Signature database implementation
impl SignatureDatabase {
    //  new — 空のデータベースを作成しデフォルトシグネチャをロード
    pub fn new() -> Self {
        let mut db = Self {
            signatures: HashMap::new(),
        };
        
        db.load_default_signatures();
        db
    }

    //  load_default_signatures — ビルトインシグネチャ (WordPress, Drupal) をロード
    fn load_default_signatures(&mut self) {
        let sigs = vec![
            VulnSignature {
                id: "OXIDE-001".to_string(),
                name: "WordPress Detected".to_string(),
                severity: "Info".to_string(),
                pattern: r"\bwp-content\b|\bwordpress\b".to_string(),
                description: "WordPress installation detected".to_string(),
                remediation: "Ensure WordPress is kept updated".to_string(),
            },
            VulnSignature {
                id: "OXIDE-002".to_string(),
                name: "Drupal CMS Detected".to_string(),
                severity: "Info".to_string(),
                pattern: r"\bdrupal\b|\bDrupal\b".to_string(),
                description: "Drupal CMS detected".to_string(),
                remediation: "Ensure Drupal is kept updated".to_string(),
            },
        ];

        for sig in sigs {
            self.signatures.insert(sig.id.clone(), sig);
        }
    }

    //  all — 全シグネチャへの参照を取得
    //  Returns reference to all signatures
    pub fn all(&self) -> &HashMap<String, VulnSignature> {
        &self.signatures
    }

    //  add — 新しいシグネチャを追加
    //  Adds a new signature to the database
    pub fn add(&mut self, sig: VulnSignature) {
        self.signatures.insert(sig.id.clone(), sig);
    }
}

impl Default for SignatureDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for SignatureDatabase {
    fn clone(&self) -> Self {
        Self {
            signatures: self.signatures.clone(),
        }
    }
}
