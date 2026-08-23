use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::binary;

fn ok_response() -> http::Response<Cow<'static, [u8]>> {
    http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .header("Access-Control-Allow-Origin", "*")
        .body(Cow::from(b"ok".to_vec()))
        .expect("static response construction")
}

pub fn build_handler(
    running: Arc<AtomicBool>,
    child_pid: Arc<Mutex<Option<u32>>>,
    proxy: tao::event_loop::EventLoopProxy<String>,
) -> impl Fn(http::Request<Vec<u8>>) -> http::Response<Cow<'static, [u8]>> {
    move |request: http::Request<Vec<u8>>| {
        let uri = request.uri().to_string();

        if uri == "oxide://CyberPunk2077-Interface/style.css" || uri == "oxide://CyberPunk2077-Interface/app.js" {
            let (body, mime) = if uri.ends_with(".css") {
                (include_str!("../CyberPunk2077-Interface/style.css").to_owned(), "text/css")
            } else {
                (include_str!("../CyberPunk2077-Interface/app.js").to_owned(), "application/javascript")
            };
            return http::Response::builder()
                .status(200)
                .header("Content-Type", mime)
                .header("Access-Control-Allow-Origin", "*")
                .body(Cow::from(body.into_bytes()))
                .expect("static response construction");
        }

        if uri.starts_with("oxide://scan/start") {
            let query = uri.trim_start_matches("oxide://scan/start").trim_start_matches('?');
            let mut params = HashMap::new();
            for pair in query.split('&') {
                if pair.is_empty() { continue; }
                if let Some((k, v)) = pair.split_once('=') {
                    params.insert(k.to_string(), urlencoding::decode(v).unwrap_or_default().to_string());
                }
            }

            let url = params.get("url").map(|s| s.as_str()).unwrap_or("");
            if !url.is_empty() && !running.load(Ordering::SeqCst) {
                running.store(true, Ordering::SeqCst);

                let binary_path = binary::find_oxide_binary();
                let p2 = proxy.clone();
                let r2 = running.clone();
                let pid2 = child_pid.clone();

                std::thread::spawn(move || {
                    binary::spawn_scan(&binary_path, &params, r2, pid2, p2);
                });
            }

            return http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .header("Access-Control-Allow-Origin", "*")
                .body(Cow::from(b"ok".to_vec()))
                .expect("static response construction");
        }

        if uri.starts_with("oxide://scan/stop") {
            running.store(false, Ordering::SeqCst);
            if let Some(pid) = *child_pid.lock().unwrap_or_else(|e| e.into_inner()) {
                let _ = std::process::Command::new("kill")
                    .args(&[pid.to_string()])
                    .spawn();
            }
            return http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .header("Access-Control-Allow-Origin", "*")
                .body(Cow::from(b"stopped".to_vec()))
                .expect("static response construction");
        }

        if uri == "oxide://window/close" {
            let _ = proxy.send_event("cmd:close".to_string());
            return ok_response();
        }
        if uri == "oxide://window/minimize" {
            let _ = proxy.send_event("cmd:minimize".to_string());
            return ok_response();
        }
        if uri == "oxide://window/maximize" {
            let _ = proxy.send_event("cmd:maximize".to_string());
            return ok_response();
        }
        if uri == "oxide://window/drag" {
            let _ = proxy.send_event("cmd:drag".to_string());
            return ok_response();
        }

        http::Response::builder()
            .status(404)
            .header("Access-Control-Allow-Origin", "*")
            .body(Cow::from(b"NOT FOUND".to_vec()))
            .expect("static response construction")
    }
}
