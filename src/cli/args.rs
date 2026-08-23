// ----------------------------------------------------------------------------
//  args.rs — CLI argument parsing
// ----------------------------------------------------------------------------
//  Defines the CliArgs struct using clap derive macros to parse command-line
//  arguments for the OXIDE scanner. Handles validation, clamping, and default
//  values for all scan configuration parameters including targets, threads,
//  modules, output format, and network options.
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
//  引数電脳解析 / argument parsing flow overview
//   CliArgs構造体 — clap::Parserを継承してコマンドライン引数を定義 / struct inherits clap::Parser
//  ターゲット指定 / target specification — url, multiattack
//  スレッド電脳制御 / thread control — threads, jobs
//  電脳走査強度 / scan intensity — exploitation_level, payload_limit
//  出力電脳制御 / output control — output, format, silent_mode, verbose
//  電脳網設定 / network settings — user_agent, cookie, header, proxy
//  HTTP動作 / HTTP behaviour — follow_redirects, max_redirects, rate_limit, insecure
//  モジュール電脳制御 / module control — modules, exclude, list_modules
//  電脳収集 / crawling — crawl_depth, max_urls, headless
//  高度な機能 / advanced features — zeroday, active, train, session, download, resume
//  マルチターゲット / multi-target — multiattack, duration
//  parse_args() — 引数を検証・クランプして安全な範囲に収める / validates and clamps args
//  .txtファイルの展開 / expands .txt file references into URL lists
//  threadsを1–100にクランプ / clamps thread count to safe range
//  exploitation_levelを1–100にクランプ / clamps exploitation level
//  payload_limitを最大500にクランプ / clamps payload limit
//  jobsを1–50にクランプ / clamps job count
//  format文字列を検証 / validates output format string
//  crawl_depthを最大10にクランプ / clamps crawl depth
//  max_urlsを最大10000にクランプ / clamps max URLs
//
use clap::Parser;

use crate::cli::parser::Parser as ArgParser;

#[derive(Parser, Debug, Clone)]
#[command(name = "oxide")]
#[command(author, version = "8.7.2-community-edition", about = "OXIDE Community Edition — Open eXtensible Intelligence & Detection Engine", long_about = None)]
pub struct CliArgs {
    #[arg(help_heading = "Target", short, long, help = "Target URL or URLs for security scanning.
                    Accepts up to 3 targets when used with --multiattack.
                    Supports .txt files as input (one URL per line).
                    \x1B[38;2;139;184;92m\x1B[3mUsage: -u https://example.com\x1B[0m")]
    pub url: Vec<String>,

    #[arg(help_heading = "Scan Control", short, long, help = "Number of concurrent worker threads for scanning.
                    Higher values increase scan speed but may trigger
                    rate limiting or WAF detection. Range: 1-100
                      \x1B[38;2;139;184;92m\x1B[3mUsage: --threads 20\x1B[0m", default_value_t = 5)]
    pub threads: usize,

    #[arg(help_heading = "Scan Control", long, help = "Scan intensity level from 1 (gentle) to 100 (aggressive).
                    Controls payload depth, timing, and evasion behaviour.
                    Higher levels send more complex payload combinations.
                     \x1B[38;2;139;184;92m\x1B[3mUsage: --exploitation-level 75\x1B[0m", default_value_t = 25)]
    pub exploitation_level: u8,

    #[arg(help_heading = "Scan Control", long, help = "Maximum number of URLs to test per scan module.
                    Controls scan scope by limiting how many crawled
                    URLs each module will test against.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --payload-limit 100\x1B[0m", default_value_t = 100)]
    pub payload_limit: usize,

    #[arg(help_heading = "Output", short, long, help = "File path to save the scan report.
                    If not specified, output is printed to stdout.
                    Used together with --format to control output type.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: -o report.json\x1B[0m")]
    pub output: Option<String>,

    #[arg(help_heading = "Output", short, long, help = "Report output format selection.
                    Supports: json (structured), html (visual report),
                    csv (tabular), xml (interoperable).
                    \x1B[38;2;139;184;92m\x1B[3mUsage: -f html\x1B[0m", default_value_t = String::from("json"))]
    pub format: String,

    #[arg(help_heading = "HTTP Session", long, help = "Custom HTTP User-Agent header string.
                    Overrides the default browser-like User-Agent.
                    Useful for mimicking specific client profiles.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --user-agent \"Mozilla/5.0 ...\"\x1B[0m")]
    pub user_agent: Option<String>,

    #[arg(help_heading = "HTTP Session", long, help = "HTTP Cookie header value to include with requests.
                    Format: name=value; name2=value2.
                    Used for authenticated scanning or session persistence.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --cookie \"session=abc123\"\x1B[0m")]
    pub cookie: Option<String>,

    #[arg(help_heading = "HTTP Session", long, help = "Custom HTTP headers to append to each request.
                    Format: \"Header-Name: value\". Specify multiple
                    times for additional headers.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --header \"X-Custom: value\"\x1B[0m")]
    pub header: Vec<String>,

    #[arg(help_heading = "Output", short, long, help = "Enable detailed logging of scan progress and findings.
                    Shows per-request status, module execution order,
                    and intermediate detection results.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: -v\x1B[0m")]
    pub verbose: bool,

    #[arg(help_heading = "Scan Control", long, help = "Maximum requests per second rate limit.
                    Use 0 for unlimited speed. Higher limits may
                    cause connection drops on rate-limited targets.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --rate-limit 10\x1B[0m", default_value_t = 0)]
    pub rate_limit: u64,

    #[arg(help_heading = "Scan Control", long, help = "Automatically follow HTTP 3xx redirect responses.
                    Disable to inspect intermediate redirect responses
                    rather than following to the destination.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --follow-redirects\x1B[0m")]
    pub follow_redirects: bool,

    #[arg(help_heading = "Scan Control", long, help = "Maximum number of consecutive redirects to follow.
                    Prevents infinite redirect loops during scanning.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --max-redirects 5\x1B[0m", default_value_t = 10)]
    pub max_redirects: u32,

    #[arg(help_heading = "HTTP Session", long, help = "Disable SSL/TLS certificate validation.
                    Allows scanning hosts with self-signed or
                    expired certificates. Use with caution.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --insecure\x1B[0m")]
    pub insecure: bool,

    #[arg(help_heading = "HTTP Session", long, help = "Route all scan traffic through Burp Suite Pro proxy.
                    Automatically sets proxy to http://127.0.0.1:8080
                    and disables certificate validation for HTTPS inspection.
                    Requires Burp Suite running with default proxy listener.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --burp\x1B[0m")]
    pub burp: bool,

    #[arg(help_heading = "Modules", long, help = "Modules to enable (comma-separated).
            Default: sqli,xss,lfi,fuzz,ssti (5 core vuln checks).
            Use 'all' to run every module.
            \x1B[38;2;139;184;92m\x1B[3mUsage: --modules sqli,xss,lfi,ssti\x1B[0m
            \x1B[38;2;139;184;92m\x1B[3mFull scan: --modules all\x1B[0m
            \x1B[38;2;232;57;41m\x1B[3m走査モジュール一覧: --list-modules で朱色の木構造を表示\x1B[0m")]
    pub modules: Option<String>,

    #[arg(help_heading = "Crawling", long, help = "Maximum link depth for URL crawling and discovery.
                    Depth 1 scans only the starting page. Higher
                    values follow links recursively.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --crawl-depth 5\x1B[0m", default_value_t = 3)]
    pub crawl_depth: u8,

    #[arg(help_heading = "Crawling", long, help = "Maximum number of unique pages to crawl and analyze.
                    Helps limit scan scope and duration on large
                    sites with many linked pages.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --max-urls 500\x1B[0m", default_value_t = 100)]
    pub max_urls: usize,

    #[arg(help_heading = "Output", long, help = "Reduce console output to warnings and findings only.
                    Suppresses progress indicators, status bars,
                    and per-module diagnostic messages.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --silent-mode\x1B[0m")]
    pub silent_mode: bool,

    #[arg(help_heading = "Advanced", long, help = "Automatically download sensitive files discovered
                    during scanning. Includes database dumps, config
                    files, backup archives, and credential stores.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --download\x1B[0m")]
    pub download: bool,

    #[arg(help_heading = "Modules", long, help = "Comma-separated list of module names to skip.
                    Excluded modules will not run during the scan.
                    Useful for disabling noisy or irrelevant checks.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --exclude engine,static\x1B[0m")]
    pub exclude: Option<String>,

    #[arg(help_heading = "Advanced", long, help = "Enable experimental zero-day vulnerability detection.
                    Uses ML-based anomaly detection to identify
                    unknown attack vectors and behavioural deviations.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --zeroday\x1B[0m")]
    pub zeroday: bool,

    #[arg(help_heading = "Advanced", long, help = "Active TCP fingerprinting via raw packet injection.
                    Requires root/sudo privileges for creating
                    raw sockets. Identifies OS and service details.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --active\x1B[0m")]
    pub active: bool,

    #[arg(help_heading = "Advanced", long, help = "Train the zero-day ML classifier by scanning with
                    all modules and collecting labelled response data
                    for baseline anomaly detection model training.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --train\x1B[0m")]
    pub train: bool,

    #[arg(help_heading = "Advanced", long, help = "Enable session security analysis module.
                    Tests for cookie flag misconfiguration, session
                    fixation vulnerabilities, and predictability.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --session\x1B[0m")]
    pub session: bool,

    #[arg(help_heading = "Advanced", long, help = "Multi-target parallel scan mode for up to 3 URLs.
                    Distributes threads adaptively across targets
                    based on responsiveness and scan progress.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --multiattack\x1B[0m")]
    pub multiattack: bool,

    #[arg(help_heading = "Scan Control", long, help = "Maximum scan runtime in seconds. Set to 0 for
                    unlimited duration. Scan stops when the timeout
                    is reached, regardless of completion status.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --duration 300\x1B[0m", default_value_t = 0)]
    pub duration: u64,

    #[arg(help_heading = "Modules", long, help = "Print all available scan modules and their
                    descriptions, then exit without scanning.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --list-modules\x1B[0m")]
    pub list_modules: bool,

    #[arg(long, hide = true, help = "Compile cgi_database/oxide_tests.db into the encrypted
                    runtime database oxide_tests.db.enc (AES-256-GCM).
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --compile-enc\x1B[0m")]
    pub compile_enc: bool,

    #[arg(help_heading = "Crawling", short = 'j', long = "jobs", help = "Number of parallel workers for URL
                    crawling and discovery operations.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: -j 4\x1B[0m", default_value_t = 2)]
    pub jobs: usize,

    #[arg(help_heading = "Crawling", long, help = "Enable headless Chromium browser for JavaScript-heavy
                    crawling. Renders pages in a real browser engine
                    to discover client-side routes and dynamic content.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --headless\x1B[0m")]
    pub headless: bool,

    #[arg(help_heading = "Advanced", long, default_value_t = true, help = "Auto-resume from the last saved checkpoint
                    for this target. Enabled by default in all scans —
                    completed URLs and findings are restored and the
                    scan continues where it left off.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --resume  [default: enabled]\x1B[0m")]
    pub resume: bool,

    #[arg(help_heading = "Advanced", long, help = "Disable automatic checkpoint resume for this scan.
                    Forces a completely fresh scan even if a saved
                    checkpoint exists for the target.
                    \x1B[38;2;139;184;92m\x1B[3mUsage: --no-resume\x1B[0m")]
    pub no_resume: bool,
}

impl CliArgs {
    /// Checkpoint resume is on by default in every scan; --no-resume opts out.
    pub fn resume_enabled(&self) -> bool {
        self.resume && !self.no_resume
    }

    pub fn parse_args() -> anyhow::Result<Self> {
        let mut args = Self::parse();

        if args.list_modules {
            return Ok(args);
        }

        if args.compile_enc {
            return Ok(args);
        }

        if args.url.is_empty() {
            anyhow::bail!("No target URL provided. Use: oxide --url <URL>");
        }

        // Expand .txt file references: -u targets.txt reads lines as URLs
        let mut expanded = Vec::new();
        for u in &args.url {
            if u.ends_with(".txt") && std::path::Path::new(u).exists() {
                let content = std::fs::read_to_string(u)
                    .map_err(|e| anyhow::anyhow!("Failed to read target file '{}': {}", u, e))?;
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        expanded.push(line.to_string());
                    }
                }
            } else {
                expanded.push(u.clone());
            }
        }
        args.url = expanded;

        // Validate and clamp threads to safe range (1-100)
        if args.threads > 100 {
            eprintln!("[WARN] threads clamped to 100 (was {})", args.threads);
            args.threads = 100;
        }
        if args.threads < 1 {
            eprintln!("[WARN] threads raised to 1 (was {})", args.threads);
            args.threads = 1;
        }

        // Clamp exploitation level
        if args.exploitation_level > 100 {
            eprintln!("[WARN] exploitation_level clamped to 100 (was {})", args.exploitation_level);
            args.exploitation_level = 100;
        }

        // Clamp payload limit
        if args.payload_limit > 500 {
            eprintln!("[WARN] payload_limit clamped to 500 (was {})", args.payload_limit);
            args.payload_limit = 500;
        }

        // Validate jobs
        if args.jobs < 1 {
            eprintln!("[WARN] jobs raised to 1 (was {})", args.jobs);
            args.jobs = 1;
        }
        if args.jobs > 50 {
            eprintln!("[WARN] jobs clamped to 50 (was {})", args.jobs);
            args.jobs = 50;
        }

        // Validate output format
        match args.format.as_str() {
            "json" | "html" | "csv" | "xml" => {}
            _ => anyhow::bail!("Invalid output format '{}'. Valid: json, html, csv, xml", args.format),
        }

        // Clamp crawl depth
        if args.crawl_depth > 10 {
            eprintln!("[WARN] crawl_depth clamped to 10 (was {})", args.crawl_depth);
            args.crawl_depth = 10;
        }

        // Clamp max_urls
        if args.max_urls > 10_000 {
            eprintln!("[WARN] max_urls clamped to 10000 (was {})", args.max_urls);
            args.max_urls = 10_000;
        }

        Ok(args)
    }

    pub fn get_modules(&self) -> Vec<String> {
        match &self.modules {
            Some(m) => ArgParser::parse_modules(m),
            None => vec![
                "sqli".to_string(),
                "xss".to_string(),
                "lfi".to_string(),
                "fuzz".to_string(),
                "ssti".to_string(),
            ],
        }
    }

    pub fn get_excluded(&self) -> Vec<String> {
        match &self.exclude {
            Some(e) => ArgParser::parse_modules(e),
            None => vec![],
        }
    }

    pub fn target_url(&self) -> &str {
        self.url.first().map(|s| s.as_str()).unwrap_or("http://localhost")
    }

    pub fn target_count(&self) -> usize {
        self.url.len()
    }

    pub fn multiattack_enabled(&self) -> bool {
        self.multiattack && self.url.len() > 1
    }
}
