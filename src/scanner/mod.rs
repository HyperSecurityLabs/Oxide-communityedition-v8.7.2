// ----------------------------------------------------------------------------
//  mod.rs — scanner module collection
// ----------------------------------------------------------------------------
//  Aggregates all security scanner submodules into a single module. Each
//  submodule implements a distinct vulnerability detection technique — SQLi,
//  XSS, LFI, command injection, CORS, TLS, and more — for comprehensive
//  web application security assessment.
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

//  モジュール構成 / module composition:
//   Each submodule implements a distinct vulnerability detection technique.
//    SQLi     (sqli_scanner, blind_sqli_scanner) — error/time/boolean/union
//    XSS      (xss_scanner) — reflected/DOM/stored
//    LFI      (lfi_scanner, path_traversal_scanner) — inclusion/traversal
//    CMD      (cmd_injection_scanner) — OS command injection
//    CORS     (cors_scanner) — misconfiguration analysis
//    TLS      (tls_scanner) — protocol/certificate assessment
//    DB       (db_fingerprinter) — database type fingerprinting
//    CREDS    (default_creds_scanner) — default credential testing
//    CF       (hypersecurity_cf) — Cloudflare/WAF bypass analysis
//    PREC     (precision) — CGI false positive elimination
//    COMMON   (common_app_scanner) — Nikto-style path enumeration (6000+ tests)
// Scanner module for performing security tests
pub mod blind_sqli_scanner;
pub mod hypersecurity_cf;
pub mod cmd_injection_scanner;
pub mod common_app_scanner;
pub mod cors_scanner;
pub mod db_fingerprinter;
pub mod default_creds_scanner;
pub mod lfi_scanner;
pub mod path_traversal_scanner;
pub mod precision;
pub mod sqli_scanner;
pub mod tls_scanner;
pub mod xss_scanner;
