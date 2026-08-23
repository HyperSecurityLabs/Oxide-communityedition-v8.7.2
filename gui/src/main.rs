mod binary;
mod protocol;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::window::WindowBuilder;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;
#[cfg(not(target_os = "linux"))]
use wry::WebViewBuilder;

fn webview_builder(window: &tao::window::Window) -> wry::WebViewBuilder<'_> {
    #[cfg(target_os = "linux")]
    {
        let vbox = window.default_vbox().expect("Failed to get vbox");
        WebViewBuilderExtUnix::new_gtk(vbox)
    }
    #[cfg(not(target_os = "linux"))]
    {
        WebViewBuilder::new(window)
    }
}

fn embed_html() -> String {
    let css = include_str!("../CyberPunk2077-Interface/style.css");
    let js = include_str!("../CyberPunk2077-Interface/app.js");
    let html = include_str!("../CyberPunk2077-Interface/index.html");
    html.replace("</head>", &format!("<style>{}</style></head>", css))
        .replace("</body>", &format!("<script>{}</script></body>", js))
}

fn main() {
    let event_loop = EventLoopBuilder::<String>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("OXIDE Community v8.7.2")
        .with_inner_size(LogicalSize::new(1200.0, 780.0))
        .with_min_inner_size(LogicalSize::new(800.0, 560.0))
        .with_decorations(false)
        .build(&event_loop)
        .expect("Failed to create window");

    let running = Arc::new(AtomicBool::new(false));
    let child_pid: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(None));

    let handler = protocol::build_handler(
        running.clone(),
        child_pid.clone(),
        proxy.clone(),
    );

    let builder = webview_builder(&window)
        .with_custom_protocol("oxide".into(), handler)
        .with_html(embed_html());

    let webview = builder.build().expect("Failed to build webview");
    let wv = Arc::new(Mutex::new(webview));

    event_loop.run(move |event, _target, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(js) => {
                match js.as_str() {
                    "cmd:drag" => { let _ = window.drag_window(); }
                    "cmd:close" => { *control_flow = ControlFlow::Exit; }
                    "cmd:minimize" => { window.set_minimized(true); }
                    "cmd:maximize" => { window.set_maximized(!window.is_maximized()); }
                    _ => { let _ = wv.lock().unwrap_or_else(|e| e.into_inner()).evaluate_script(&js); }
                }
            }
            _ => {}
        }
    });
}
