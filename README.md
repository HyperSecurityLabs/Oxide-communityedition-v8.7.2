# Oxide-communityedition-v8.7.2
**Precision-forged Rust vulnerability scanner**  
HyperSecurity Offensive Labs   
FP Reduction · Zero-Day ML Anomaly Engine · WAF Massacre · Headless DOM · Distributed Cluster · Burp Suite Integration · Unified Fuzz Engine · Live Progress Board
<img width="1598" height="888" alt="oxide7" src="https://github.com/user-attachments/assets/c1581013-557a-4829-9455-f296e8e9042d" />

[![GUI](https://img.shields.io/badge/_GUI-Launch_OXIDE-C0392B?style=for-the-badge&logo=electron&logoColor=000&labelColor=FFE8E0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
[![Termux](https://img.shields.io/badge/_Termux-Ready-000000?style=for-the-badge&logo=android&logoColor=FFF&labelColor=2E2E2E)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
[![Forums](https://img.shields.io/badge/_Forums-Community-2E8B7A?style=for-the-badge&logo=discourse&logoColor=000&labelColor=EDF5E0)](https://hypersecurityoffseclabs.great-site.net/forums/index.php)
[![Rust](https://img.shields.io/badge/_Rust-2021-E8A0BF?style=for-the-badge&logo=rust&logoColor=000&labelColor=FFF0F0)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/_Platform-WinLinux-2EA9DF?style=for-the-badge&logo=linux&logoColor=000&labelColor=E8F4FD)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)
[![ML](https://img.shields.io/badge/_ML_Stack-7B68AE?style=for-the-badge&labelColor=EDEAF8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)
[![License](https://img.shields.io/badge/_License-GPL--3.0--only-8B81C3?style=for-the-badge&logo=libreoffice&logoColor=000&labelColor=7B68AE)](../LICENSE)
[![Downloads](https://img.shields.io/badge/_Downloads-v8.7.2-91989F?style=for-the-badge&logo=github&logoColor=000&labelColor=F0F0F0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
[![Zero-Day ML](https://img.shields.io/badge/_Zero--Day_ML-Anomaly_Engine-C46B5A?style=for-the-badge&logo=smart&logoColor=000&labelColor=FFF0F0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)
> [![Android](https://img.shields.io/badge/_Android-arm64_·_armv7_·_x86__64-3DDC84?style=for-the-badge&logo=android&logoColor=000)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
> [![Kali](https://img.shields.io/badge/_Kali_Linux-x86__64·arm64-1E3A5F?style=for-the-badge&logo=kalilinux&logoColor=FFF)](https://www.kali.org/)
> [![Windows](https://img.shields.io/badge/_Windows-x86__64.exe-0078D4?style=for-the-badge&logo=windows&logoColor=FFF)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)

</div>

---

[![Unauthorized use prohibited](https://img.shields.io/badge/UNATHORIZED_USE-PROHIBITED-C0392B?style=for-the-badge&labelColor=1A1A1A)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

OXIDE is a weapon-grade security tool. In the wrong hands, its capabilities cause severe disruption. You are solely responsible for how you use it.
- DO NOT scan systems you do not own or lack written authorization to test.
- DO NOT use for illegal access, data theft, or system damage.
- DO NOT extract or reimplement its detection logic in malicious software.
- DO use for authorized penetration testing, CTFs, labs, and security research.
> Violators assume full legal liability. HSOL bears no responsibility for misuse.
 
---

[![Final Release](https://img.shields.io/badge/8.7.2-RELEASE-FFB11B?style=for-the-badge&labelColor=1A1A1A)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

> v8.7.2 — Hardened production release. Burp Suite integration, module isolation, embedded TLS cert, zero dead code, zero silent errors, async-safe mutexes, full --silent-mode, full --duration, and 50+ fixes across the codebase.

> Every star brings OXIDE closer to `sudo apt install oxide`. Built for Kali, tested on Kali — destined for the official Kali Linux repositories. 

---

[![About](https://img.shields.io/badge/_About-OXIDE-1E3A5F?style=for-the-badge&logo=github&logoColor=000&labelColor=E8F0F8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

Modular security toolkit combining traditional vulnerability scanning with ML-based anomaly detection. Built in Rust for Kali Linux.

[![Rust](https://img.shields.io/badge/_Rust_2021-C0392B?style=for-the-badge&logo=rust&logoColor=000&labelColor=FFE8E0)](https://www.rust-lang.org/)

---

[![Installation](https://img.shields.io/badge/_Installation-Quick_Start-2EA9DF?style=for-the-badge&logo=terminal&logoColor=000&labelColor=E8F4FD)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

```bash
sudo apt install -y build-essential pkg-config libssl-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
git clone https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2.git
cd oxide-communityedition-v8.7.2 && cargo build --release
sudo cp target/release/oxide /usr/local/bin/
```
---

[![Scanner Modules](https://img.shields.io/badge/_Scanner_Modules-14_Engines-2E8B7A?style=for-the-badge&logo=github&logoColor=000&labelColor=E8F5E8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

| Module | Detection | Module | Detection |
|--------|-----------|--------|-----------|
| **SQLi** ⭐ | Error, boolean, time, UNION, stacked | **Blind SQLi** | Blind / time-based |
| **XSS** ⭐ | Reflected, stored, DOM | **LFI** ⭐ | File read confirmation |
| **CMDi** | Linux + Windows | **CORS** | Misconfiguration audit |
| **TLS** | Certs, protocols, ciphers | **Common App** | Nikto-style path probing |
| **Default Creds** | Known admin creds | **DB Fingerprint** | MySQL, PG, MSSQL, Oracle, SQLite |
| **Content Filter** | Keys, tokens, secrets | **Agent** | Deep AI-driven anomaly scan |
| **Fuzz** ⭐ | 6000+ payload injection | **SSTI** ⭐ | Template injection (Jinja2/Freemarker/Velocity) | **still In Development**|
> Nosql Injection Still in development needs wiring 

⭐ = Default modules (runs without `--modules` flag)

---

[![Zero-Day ML](https://img.shields.io/badge/_Zero--Day_ML-Anomaly_Engine-C46B5A?style=for-the-badge&logo=smart&logoColor=000&labelColor=FFF0F0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

**Pipeline:** Crawl → ML Analysis + Auto-Exploit → Fuzz → HPP Detection → Report

**Proof of code** (from `src/zero_day/classifier.rs`):

```rust
// Feature Extraction — custom 30-dim response vectors (features.rs:52)
pub struct ResponseFeatures {
    pub body_length: usize,
    pub entropy: f64,
    pub has_sql_error: bool,
    pub security_header_count: usize,
    pub status_code: u16,
    pub is_error_status: bool,
    // ... 30 fields total
}

// Random Forest — smartcore (classifier.rs:151)
RandomForestClassifier::fit(&x_train, &y_train, Default::default())

// SVM — smartcore RBF kernel (classifier.rs:312)
SVC::fit(x, &y_svm, &SVCParameters::default().with_c(100.0).with_kernel(Kernels::rbf()))

// Baseline Profiling — statistical (classifier.rs:334)
// Posterior odds = prior_odds × LR₁ × LR₂ × ...
// Prior: ~10% endpoints vulnerable → prior_odds = 0.111
// SQL error string LR: 15, Stack trace LR: 8, High entropy+error LR: 1.8
```

Auto-exploit: SQLi · XSS · LFI · CMDi · SSTI · WAF Bypass (12 vendors)

---

[![Advanced](https://img.shields.io/badge/_Advanced-Capabilities-2EA9DF?style=for-the-badge&logo=github&logoColor=000&labelColor=E8F4FD)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

**Session & Auth** — Cookie, Bearer, Basic, API Key, JWT, OAuth2 · Hijack testing
**JS Crawling** — Headless Chrome · SPA routes · JS URL extraction
**API Fuzzer** — REST + GraphQL · 7 methods · 6 content types
**WebSocket** — SQLi, XSS, CMDi, path traversal, JSON injection, 
**Recon** — TCP fingerprinting · OS detection · Banner grabbing · DNS · WHOIS

---

[![GUI Frontend](https://img.shields.io/badge/_GUI_Frontend-Desktop_App-C46B5A?style=for-the-badge&logo=electron&logoColor=000&labelColor=FFF0F0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

<img width="1597" height="873" alt="gui3" src="https://github.com/user-attachments/assets/d40da638-5974-4d47-999a-aec2da40e284" />
A Frameless Native desktop GUI built with **WRY** (WebView2/WebKit) + **TAO** (windowing). Frameless window, scan presets, config panel, module toggles, live terminal console, status badge, About modal. Keyboard shortcuts: `Ctrl+Enter` start, `Escape` stop, `F12` DevTools.

```bash
cd gui && cargo build --release && sudo cp target/release/oxide-gui /usr/local/bin/ && oxide-gui
```

---

[![CLI Reference](https://img.shields.io/badge/_CLI-Full_Reference-1E3A5F?style=for-the-badge&logo=terminal&logoColor=000&labelColor=E8F0F8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

| Flag | Default | Purpose | Flag | Default | Purpose |
|------|---------|---------|------|---------|---------|
| `--url` | required | Target(s) or file | `--modules` | `sqli,xss,lfi,fuzz,ssti` | `all` or comma-sep |
| `--burp` | false | Route via Burp 127.0.0.1:8080 | `--multiattack` | false | Multi-target |
| `--zeroday` | false | ML anomaly mode | `--headless` | false | Chrome JS |
| `--threads` | 5 | Concurrent workers | `--duration` | 0 | Scan time limit (seconds) |
| `--active` | false | TCP fingerprinting | `--exploitation-level` | 25 | Aggression (1–100) |
| `--silent-mode` | false | Quiet output | `--insecure` | false | Skip TLS verify |
| `--session` | false | Session hijack | `--train` | false | Train ML |
| `--list-modules` | — | Show all modules | `--payload-limit` | 100 | Max URLs per module |

Config: `oxide-config.toml` for persistent settings.

---

[![Reports](https://img.shields.io/badge/_Reports-Formats-7B68AE?style=for-the-badge&logo=github&logoColor=000&labelColor=EDEAF8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

| Format | Use Case |
|--------|----------|
| HTML | Human review |
| JSON | Machine-parsable — automation / pipelines |
| CSV | Spreadsheet-ready — data analysis |
| XML | Standard schema — tool integration |

Auto-saved to `reports/oxide_<timestamp>.*`

---

[![Changelog](https://img.shields.io/badge/_Changelog-v8.7.2--community-2E8B7A?style=for-the-badge&logo=github&logoColor=000&labelColor=E8F5E8)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)

## v8.7.2 — Unified Engine Update

- ALL module selections (`--modules sqli` included) now flow through one chunk-based concurrent engine with the same live display
- Adaptive module chain: shows only selected scanners, extras (CMDi/SSTI/NoSQL) on `fuzz`/`all`
- **SSTI wired in** — template math-reflection detection ({{7*7}}→49) with baseline FP guard
- High-yield parameter mining for parameterless URLs, capped by exploitation level
- Live progress: wave meter, 20-cell bar, request counter, total estimate, ETA, param tags
- Destructive SQLi-D payloads require `--exploitation-level 60+`

### Scanner Modernization
- SQLi: sqlmap-style pipeline (detect → confirm → identify), per-URL findings, DBMS fingerprint only after confirmed hits
- XSS/LFI/CMDi/Blind-SQLi/Path-traversal: AI `PayloadMutator` variants + silent worker mode
- LFI progressive escalation: cheap reads first, bypass phases gated by exploitation level
- HTTP client panic-free header handling; global live request ticker

## v8.7.2 — Hardened Production Release

### Burp Suite Integration
- `--burp` boolean flag routes all traffic through Burp Suite Pro at `127.0.0.1:8080`
- Auto-enables `--insecure` for Burp interception
- Burp CA certificate embedded (`src/oxide.crt`) — auto-loaded as root cert for HTTPS interception
- Startup connectivity check confirms Burp is running before scan begins
- Works with HTTP/HTTPS targets, proxy chain, and SOCKS

### Module Isolation
- Default scan runs 5 core vulnerability modules: `sqli, xss, lfi, fuzz, ssti`
- `--modules all` runs all 14 modules (full scan)
- `--modules sqli,xss` runs specific modules only
- `--list-modules` shows DEFAULT/FULL tags with usage examples

### Production Hardening (50+ fixes)
- **Zero `unwrap()` in production code** — 18 critical unwraps replaced with safe error handling
- **Zero silent error swallowing** — 17 `Err(_) => {}` sites now log via `eprintln!`
- **Zero `#[allow(dead_code)]`**, zero `todo!()`, zero HTTP header pollution
- **CRLF injection prevention** — all header values validated against `\r\n` injection
- **Async-safe mutexes** — `std::sync::Mutex` replaced with `tokio::sync::Mutex` in all async contexts
- **Cookie bug fixed** — `--cookie` now sent to targets instead of localhost
- **Content-Type default** — POST requests include `Content-Type: application/json` by default
- **`--silent-mode` fully functional** — all progress output gated by `!silent`
- **`--duration` verified** — stops scan at configured time limit
- **`--threads` default: 5** — balanced concurrency
- **`--exploitation-level` default: 25** — balanced aggression
- **Session hijack timeout detection** — `check_session_timeout()` returns findings when cookie lacks Max-Age/Expires
- **TCP fingerprinting fixed** — TTL from actual SYN-ACK, source IP via local interface
- **Version bumped to 8.7.2** across all files (Cargo.toml, args.rs, lib.rs, main.rs, db.rs, config.rs, html.rs, json.rs)

## Core Performance
- Async/await concurrent architecture: tokio-based agent pool with `join_all` parallel dispatch for high-speed multi-target scanning
- Chunk-based async fuzzing engine: concurrent payload injection across all modules with adaptive worker scaling
- Zero-copy async TCP connect scanner with `tokio::net::TcpStream` for rapid port/probe scanning
- 0 Allow Dead Code Elimilation
- 0 Unsafe memory blocks thread safety

## WAF12 Evasion Suite
- 12 evasion techniques across 4 major WAF profiles (CloudFlare, ModSecurity, AWS WAF, Imperva/Incapsula)
- Protocol-level evasion: HTTP/1.0 ↔ HTTP/2 switching, method alternation
- Encoding bypass: double URL encoding, Unicode (%uXXXX), UTF-8 NBSP injection
- Case randomization: per-character bit-masked case mutation
- Comment injection: `/**/`, `/*!`, `--`, `#` at configurable intervals
- Whitespace variation: tab, newline, NBSP, UTF-8 NBSP substitution
- Path traversal unicode: overlong UTF-8, fullwidth path sequences
- Fragmentation: payload split markers for multi-request delivery
- Header smuggling: `X-Forwarded-For`, `X-Original-Url`, `X-Real-Ip` spoofing
- JSON/XML/Multipart wrapper bypass with CDATA sections
- 12-vendor WAF fingerprinting: CloudFlare, AWS WAF, ModSecurity, F5 BIG-IP ASM, Imperva, Akamai, Sucuri, Radware, Palo Alto, Fortinet, Barracuda, Citrix

## AI/ML Vulnerability Research Engine

OXIDE 8.7.2 integrates a Rust-native machine-learning pipeline for HTTP behavioral analysis, adaptive anomaly detection, intelligent mutation, and vulnerability research.

- Random Forest + SVM ensemble powered by "smartcore", with 5-fold cross-validation for model evaluation
- 30-dimensional HTTP response feature vectors covering entropy, timing, content structure, security headers, character distributions, and SHA-256 content fingerprints
- Adaptive baseline profiling through online pattern updates with "add_normal_pattern()"
- ExploitAnalyzer for response-pattern learning, mutation prioritization, and context-aware payload selection
- PayloadMutator with 8 mutation strategies including case variation, encoding, obfuscation, comment insertion, whitespace manipulation, character substitution, concatenation, and null-byte variation
- Polyglot payload generation supporting 7 multi-context injection strategies
- Confidence-driven testing that prioritizes higher-confidence anomaly candidates for controlled validation
- Model persistence with serialized model export/import and validation
- Evidence-aware analysis linking model predictions to HTTP request/response behavior instead of treating ML predictions alone as confirmed vulnerabilities

> Important: ML confidence represents a model prediction, not proof of exploitability. OXIDE separates anomaly detection, candidate identification, and vulnerability validation to reduce false positives.

## Bayesian Confidence Scoring ***Advanced***
- `bayesian_confidence()`: sequential Bayesian update across all detection modules
- Posterior probability from evidence signals: P(V|E) = P(E|V)×P(V) / (P(E|V)×P(V) + P(E|~V)×P(~V))
- Naive Bayes multiplicative confidence in VulnerabilityClassifier: posterior odds = prior × LR_i
- Bayesian scoring integrated in SQLi, CMDi, and Hypersecurity CF bypass scanners
- Adaptive Bayesian rate-limit evasion with EMA confidence smoothing
- PatternLearner: exponential moving average Bayesian-style confidence tracking

## Levenshtein Resilient Analysis 
- `normalized_levenshtein` via `strsim` for URL deduplication with adaptive threshold (85%–97% based on exploitation level)
- `response_similarity()`: Levenshtein distance between baseline and response for diff scoring
- `response_diff_score()`: 1.0 − similarity for injection detection
- N-gram cosine similarity fallback for structural changes Levenshtein misses
- Fuzzing dedup count display: real-time Levenshtein-filtered unique payload analysis
- Exploitation level system (1–100) maps to dedup threshold, payload count, and error tolerance

## Live Progress Board (NEW)
- Unified concurrent fuzz engine — **one architecture for every module selection**: `--modules sqli`, `--modules sqli,xss`, or `--modules all` all share the same live display
- Adaptive module chain shows ONLY what you selected: `SQLi › SQLi-D › XSS` — unselected modules burn zero requests
- Real-time header: wave meter + 20-cell progress bar + `%` + request counter + total estimate + **ETA** + detections/errors + elapsed
- Live parameter tags: `>>>> ‹aid› https://target/products/?aid=...` — see exactly which parameter is under test
- Per-scanner payload estimation banner: `SQLi×8 SQLi-D×2 XSS×8 LFI×6 · 61 URLs × params → 4,218 requests`

## Parameter Mining (NEW) ***CodeByKhaninKali***
- Parameterless URLs are seeded with a curated **high-yield modern param set** (30 entries across 4 tiers)
- Tier 1 SQLi gold: `id, pid, uid, user_id, product_id, post_id, order, sort, orderby, dir, limit, offset, start`
- Tier 2 LFI/RCE sinks: `file, path, page, template, view, include, lang, doc, load, cmd, exec, run`
- Tier 3 redirect/SSRF: `redirect, next, url, return, target, continue`
- Tier 4 modern bypass: `callback, jsonp, format, debug, admin, id[]` (array syntax slips past naive WAF regex)
- Param count scales with `--exploitation-level`: <40 → 10 params, <60 → 18, 60+ → 24
> May God Help the Website What it can break But Instead God don't help Illegal Websites Make it easier For Exploitations 

## Scanning Modules
- 15 detection modules: SQLi, Blind SQLi, XSS, LFI, Path Traversal, CMD Injection, CORS, TLS, DB Fingerprinter, Default Creds, Cloudflare/WAF, Precision, Common App, Hypersecurity CF
- 10 advanced modules: API Fuzzer, Cache, Cluster (distributed), JS Crawler, Evasion, ML Detector, Plugin (FFI), Rate Limiter, Session, WebSocket
- WebSocket fuzzing: handshake injection, frame manipulation, auth bypass, 6 vulnerability types
- HPP (HTTP Parameter Pollution) detector with 8+ test payload types
- Distributed cluster scanning: master/agent TCP architecture with JSON messaging

[![Unified Fuzzing](https://img.shields.io/badge/_Fuzzing_Engine-UNIFIED_CONCURRENT-FFB11B?style=for-the-badge&labelColor=1A1A1A)]()

- Single concurrent chunk-based engine drives ALL module selections — SQLi, SQLi-D, XSS, LFI, CMDi, SSTI, NoSQL
- Adaptive depth: `--modules sqli` runs only SQLi payloads; extras (CMDi/SSTI/NoSQL) activate on `fuzz`/`all`
- Destructive SQLi-D payloads gated behind `--exploitation-level 60+` (safe by default)
- 8 payload categories: SQLi (error/union/time/boolean/stacked/WAF/noSQL/destructive), XSS, SSTI (Jinja2/Freemarker/Velocity/Smarty), LFI (path traversal/PHP wrappers), CMDi (basic/OOB/time-based/reverse shell/Windows), NoSQL, destructive SQL
- 6000+ tech-aware paths and injection templates 
- Encoder: URL/Base64/Hex/Unicode/HTML entity with mixed encoding modes
- API fuzzer: REST + GraphQL injection templates

---

[![Android](https://img.shields.io/badge/_Android-arm64_·_armv7_·_x86_64-3DDC84?style=for-the-badge&logo=android&logoColor=000&labelColor=E8F8EE)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
[![Windows](https://img.shields.io/badge/_Windows-x86__64_MSVC-0078D4?style=for-the-badge&logo=windows&logoColor=FFF&labelColor=E3F1FC)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)
[![Termux](https://img.shields.io/badge/_Termux-Ready-000000?style=for-the-badge&logo=android&logoColor=FFF&labelColor=2E2E2E)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/releases)

## Multi-Platform Builds ***Precision-forged**

OXIDE ships native binaries for every offensive workstation — desktop to pocket.

### Android (`build-android.sh`)
| ABI | Binary | Use |
|-----|--------|-----|
| `arm64-v8a` | `oxide-android-builds/arm64-v8a/oxide` | Modern phones / Termux proot |
| `armeabi-v7a` | `oxide-android-builds/armeabi-v7a/oxide` | Older 32-bit devices |
| `x86_64` | `oxide-android-builds/x86_64/oxide` | Android emulators / Chromebooks |

```bash
./build-android.sh                      # auto-detects NDK, builds all ABIs
# Termux direct: copy the arm64 binary into $PREFIX/bin and chmod +x
```

### Windows (MinGW cross-compile from Linux)
```bash
sudo apt install mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
# → target/x86_64-pc-windows-gnu/release/oxide.exe
```
See [CROSS-COMPILE.md](CROSS-COMPILE.md) for the full matrix (NDK versions, LTO profiles, troubleshooting).

> Low-RAM build note: on machines with <8GB RAM use
> `CARGO_PROFILE_RELEASE_LTO=thin CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 cargo build --release`

---

[![Build](https://img.shields.io/badge/_Build-Release-2EA9DF?style=for-the-badge&logo=rust&logoColor=000&labelColor=E8F4FD)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)

```bash
cargo clean && cargo build --release   # opt-level=3, LTO=fat, stripped, panic=abort
```

```
src/             scanner/, zero_day/, ai/, advanced/, cli/, ...
oxide-proxy/     HTTP + SOCKS4/5 proxy sub-crate
hypersecurity/   Kernel memory safety (libloading)
gui/             WRY + TAO desktop frontend
oxide-ce-debian/ DEB packaging · arch-pkg/  Arch packaging
```

---

[![Kali Linux](https://img.shields.io/badge/_Kali_Linux-Repository-7B68AE?style=for-the-badge&logo=kalilinux&logoColor=000&labelColor=EDEAF8)](https://www.kali.org/)

Targeting official Kali Linux repository: `sudo apt update && sudo apt install oxide`

| Step | Status |
|------|--------|
| Debian/Arch packaging | ✓ Complete |
| Live progress board | ✓ Complete |
| `pnet` raw socket support | ✓ Complete |
| Battle Tested | Uses Levenshtein · Burp Integration · Module Isolation |

[![Issues](https://img.shields.io/badge/_Report_Bugs-C46B5A?style=for-the-badge&logo=bugatti&logoColor=000&labelColor=FFF0F0)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2/issues)
[![Telegram](https://img.shields.io/badge/_Join_Community-7B68AE?style=for-the-badge&logo=telegram&logoColor=000&labelColor=EDEAF8)](https://t.me/hypersecurity_offsec)

---

<div align="center">

[![Star](https://img.shields.io/badge/_Star_on_GitHub-Support-2EA9DF?style=for-the-badge&logo=github&logoColor=000&labelColor=E8F4FD)](https://github.com/HyperSecurityLabs/oxide-communityedition-v8.7.2)
[![Website](https://img.shields.io/badge/_Website-HyperSec-2E8B7A?style=for-the-badge&logo=google-chrome&logoColor=000&labelColor=E8F5E8)](https://hypersecurityoffseclabs.great-site.net/)
[![Telegram](https://img.shields.io/badge/_Telegram-Community-2EA9DF?style=for-the-badge&logo=telegram&logoColor=000&labelColor=E8F4FD)](https://t.me/hypersecurity_offsec)

</div>



