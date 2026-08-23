// ----------------------------------------------------------------------------
//  db.rs — encrypted SQLite test database loader
// ----------------------------------------------------------------------------
//  Loads the encrypted SQLite database (oxide_tests.db.enc), AES-256-GCM decrypts
//  it to a secure temporary file, then opens it with rusqlite to retrieve all
//  test records. Each record contains path, method, expected status, content
//  indicators, severity, category, and remediation data used to drive the
//  vulnerability scanning engine.
//
//  --- Developers ---------------------------------------------------------------
//  khaninkali             — разработчик / core engineer (Rust backend, logic)
//  Lyara Koroleva         — дизайнер / blazing fast CLI & visual design
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
use anyhow::{Result, anyhow};
use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use sha2::{Sha256, Digest};
use std::io::Write;
use std::path::Path;

// Database directory name
pub const DB_DIR: &str = "cgi_database";
pub const DB_FILE: &str = "oxide_tests.db";
pub const DB_ENC_FILE: &str = "oxide_tests.db.enc";

/// Derive a 256-bit AES key from the master secret using SHA-256.
fn derive_key() -> Key<Aes256Gcm> {
    let master = b"OXIDE::v8.7.2-community-edition::HyperSecurityOffensiveLabs";
    let mut hasher = Sha256::new();
    hasher.update(master);
    let hash = hasher.finalize();
    let mut key = Key::<Aes256Gcm>::default();
    key.copy_from_slice(&hash);
    key
}

/// Decrypt an AES-256-GCM encrypted file to a secure temporary file.
/// The temp file is auto-deleted when the returned handle is dropped.
pub fn decrypt_to_temp(enc_path: &Path) -> Result<(tempfile::NamedTempFile, std::path::PathBuf)> {
    let data = std::fs::read(enc_path)
        .map_err(|e| anyhow!("Failed to read encrypted DB '{}': {}", enc_path.display(), e))?;

    // Format: [12-byte nonce][ciphertext][16-byte tag]
    if data.len() < 28 {
        return Err(anyhow!("Encrypted file too short ({} bytes) — corrupt or wrong format", data.len()));
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let key = derive_key();
    let cipher = Aes256Gcm::new(&key);

    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("AES-256-GCM decryption failed — wrong key or corrupt file"))?;

    if decrypted.len() < 16 || &decrypted[..16] != b"SQLite format 3\x00" {
        return Err(anyhow!("Decrypted data is not a valid SQLite database"));
    }

    let mut tmp = tempfile::NamedTempFile::new()
        .map_err(|e| anyhow!("Failed to create temp file: {}", e))?;
    std::io::Write::write_all(&mut tmp, &decrypted)
        .map_err(|e| anyhow!("Failed to write decrypted data: {}", e))?;
    tmp.flush()
        .map_err(|e| anyhow!("Failed to flush temp file: {}", e))?;

    let path = tmp.path().to_path_buf();
    Ok((tmp, path))
}

pub fn load_all_rows(db_path: &Path) -> Result<Vec<(String, String, String, String, String, String, String, String, String, bool)>> {
    let conn = rusqlite::Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT path, method, expected_status, content_indicators,
                severity, category, title, description, remediation, download_flag
         FROM tests ORDER BY id"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
            row.get::<_, String>(7)?,
            row.get::<_, String>(8)?,
            row.get::<_, i32>(9)? != 0,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

/// Convenience: decrypt + load all rows in one call.
/// Temp file is cleaned up automatically when the handle is dropped.
pub fn decrypt_and_load(enc_path: &Path) -> Result<Vec<(String, String, String, String, String, String, String, String, String, bool)>> {
    let (_tmp, path) = decrypt_to_temp(enc_path)?;
    let result = load_all_rows(&path);
    // _tmp is dropped here, auto-deleting the temp file
    result
}

pub fn encrypt_file(plain_path: &Path, enc_path: &Path) -> Result<()> {
    let plain = std::fs::read(plain_path)
        .map_err(|e| anyhow!("Failed to read '{}': {}", plain_path.display(), e))?;

    if !plain.starts_with(b"SQLite format 3\x00") {
        return Err(anyhow!("'{}' is not a SQLite database", plain_path.display()));
    }

    use aes_gcm::aead::rand_core::RngCore;
    let mut nonce_bytes = [0u8; 12];
    aes_gcm::aead::OsRng.fill_bytes(&mut nonce_bytes);
    let cipher = Aes256Gcm::new(&derive_key());
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plain.as_ref())
        .map_err(|_| anyhow!("AES-256-GCM encryption failed"))?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);

    std::io::BufWriter::new(std::fs::File::create(enc_path)?)
        .write_all(&out)
        .map_err(|e| anyhow!("Failed to write '{}': {}", enc_path.display(), e))
}
