use std::io::BufRead;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

fn sanitize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let b = s.as_bytes();
    while i < b.len() {
        let c = b[i];
        // strip ANSI escape sequences: ESC [ params* finalbyte
        if c == 0x1b && i + 1 < b.len() && b[i + 1] == b'[' {
            i += 2;
            while i < b.len() {
                let c2 = b[i];
                if c2 == 0x1b { break; }
                if c2 >= 0x40 && c2 <= 0x7e { i += 1; break; }
                i += 1;
            }
            continue;
        }
        // strip control characters (0x00-0x1f, 0x7f) except tab
        if c <= 0x1f && c != 0x09 || c == 0x7f {
            i += 1;
            continue;
        }
        out.push(c as char);
        i += 1;
    }
    out
}

pub fn find_oxide_binary() -> std::path::PathBuf {
    let system_paths = [
        "/usr/bin/oxide",
        "/usr/local/bin/oxide",
        "/usr/bin/oxide-ce",
        "/usr/local/bin/oxide-ce",
        "/opt/oxide/oxide",
    ];
    for p in &system_paths {
        if std::path::Path::new(p).exists() {
            return std::path::PathBuf::from(p);
        }
    }

    let cwd = std::env::current_dir().unwrap_or_default();
    let candidates = [
        "../../target/release/oxide",
        "../target/release/oxide",
        "target/release/oxide",
        "../../target/release/oxide-ce",
        "../target/release/oxide-ce",
        "target/release/oxide-ce",
        "oxide",
        "oxide-ce",
    ];
    for rel in &candidates {
        let p = cwd.join(rel);
        if p.exists() { return p; }
    }
    if let Some(parent) = cwd.parent() {
        for rel in &candidates {
            let p = parent.join(rel);
            if p.exists() { return p; }
        }
    }

    for p in &system_paths {
        if std::path::Path::new(p).exists() {
            return std::path::PathBuf::from(p);
        }
    }
    std::path::PathBuf::from("oxide")
}

fn needs_root(params: &std::collections::HashMap<String, String>) -> bool {
    let root_flags = ["active", "train", "recon"];
    for flag in &root_flags {
        if let Some(v) = params.get(*flag) {
            if v == "true" { return true; }
        }
    }
    false
}

pub fn spawn_scan(
    binary: &std::path::Path,
    params: &std::collections::HashMap<String, String>,
    running: Arc<AtomicBool>,
    child_pid: Arc<Mutex<Option<u32>>>,
    proxy: tao::event_loop::EventLoopProxy<String>,
) {
    let elevate = needs_root(params);

    let mut cmd: Command;
    if elevate {
        cmd = Command::new("pkexec");
        cmd.arg(binary);
        let _ = proxy.send_event(format!("ao({},'warn')",
            serde_json::to_string(">> ACTIVE MODE: elevating via pkexec (root password required)").unwrap_or_default()));
    } else {
        cmd = Command::new(binary);
    }

    let url = params.get("url").cloned().unwrap_or_default();

    cmd.arg("-u").arg(&url);

    macro_rules! opt {
        ($key:expr, $flag:expr) => {
            if let Some(v) = params.get($key) {
                if !v.is_empty() { cmd.arg($flag).arg(v); }
            }
        };
    }

    opt!("threads", "-t");
    opt!("level", "--exploitation-level");
    opt!("payloads", "--payload-limit");
    opt!("depth", "--crawl-depth");
    opt!("maxurls", "--max-urls");
    opt!("jobs", "-j");
    opt!("format", "-f");
    opt!("output", "-o");
    opt!("ua", "--user-agent");
    opt!("cookie", "--cookie");
    opt!("burp", "--burp");
    opt!("exclude", "--exclude");

    if let Some(v) = params.get("modules") {
        if !v.is_empty() { cmd.arg("--modules").arg(v); }
    }

    if let Some(v) = params.get("rate") {
        if let Ok(n) = v.parse::<u32>() { if n > 0 { cmd.arg("--rate-limit").arg(v); } }
    }
    if let Some(v) = params.get("duration") {
        if let Ok(n) = v.parse::<u32>() { if n > 0 { cmd.arg("--duration").arg(v); } }
    }
    if let Some(v) = params.get("redirects") {
        if let Ok(n) = v.parse::<u32>() { if n > 0 { cmd.arg("--max-redirects").arg(v); } }
    }

    if let Some(v) = params.get("headers") {
        if !v.is_empty() {
            for h in v.split('|') {
                let h = h.trim();
                if !h.is_empty() { cmd.arg("--header").arg(h); }
            }
        }
    }

    let flags = [
        ("follow", "--follow-redirects"),
        ("insecure", "--insecure"),
        ("verbose", "-v"),
        ("silent", "--silent-mode"),
        ("download", "--download"),
        ("zeroday", "--zeroday"),
        ("active", "--active"),
        ("train", "--train"),
        ("session", "--session"),
        ("multi", "--multiattack"),
        ("headless", "--headless"),
        ("resume", "--resume"),
        ("burp", "--burp"),
    ];
    for (key, flag) in &flags {
        if let Some(v) = params.get(*key) {
            if v == "true" { cmd.arg(flag); }
        }
    }

    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let pid = child.id();
            *child_pid.lock().unwrap_or_else(|e| e.into_inner()) = Some(pid);
            let _ = proxy.send_event(format!("setStatus('SCANNING  PID:{}','scanning')", pid));

            let stdout = child.stdout.take();
            let stderr = child.stderr.take();

            let proxy_out = proxy.clone();
            let proxy_err = proxy.clone();

            if let Some(out) = stdout {
                std::thread::spawn(move || {
                    let r = std::io::BufReader::new(out);
                    for line in r.lines() {
                        if let Ok(l) = line {
                            let clean = sanitize(&l);
                            if !clean.is_empty() {
                                let js = format!("ao({},'out')", serde_json::to_string(&clean).unwrap_or_default());
                                let _ = proxy_out.send_event(js);
                            }
                        }
                    }
                });
            }
            if let Some(err) = stderr {
                std::thread::spawn(move || {
                    let r = std::io::BufReader::new(err);
                    for line in r.lines() {
                        if let Ok(l) = line {
                            let clean = sanitize(&l);
                            if !clean.is_empty() {
                                let js = format!("ao({},'err')", serde_json::to_string(&clean).unwrap_or_default());
                                let _ = proxy_err.send_event(js);
                            }
                        }
                    }
                });
            }

            let status = child.wait();
            match status {
                Ok(s) if s.success() => {
                    let _ = proxy.send_event(format!("ao({},'ok')",
                        serde_json::to_string(">> SCAN COMPLETE — all operations finished.").unwrap_or_default()));
                }
                Ok(s) => {
                    let msg = format!("!! SCAN EXIT CODE: {}", s.code().unwrap_or(-1));
                    let _ = proxy.send_event(format!("ao({},'err')",
                        serde_json::to_string(&msg).unwrap_or_default()));
                }
                Err(e) => {
                    let msg = format!("!! SCAN ERROR: {}", e);
                    let _ = proxy.send_event(format!("ao({},'err')",
                        serde_json::to_string(&msg).unwrap_or_default()));
                }
            }
            running.store(false, Ordering::SeqCst);
            *child_pid.lock().unwrap_or_else(|e| e.into_inner()) = None;
            let _ = proxy.send_event("setRunning(false)".to_string());
        }
        Err(e) => {
            let msg = format!("!! FAILED TO LAUNCH OXIDE: {}", e);
            let _ = proxy.send_event(format!("ao({},'err')",
                serde_json::to_string(&msg).unwrap_or_default()));
            running.store(false, Ordering::SeqCst);
            let _ = proxy.send_event("setRunning(false)".to_string());
        }
    }
}
