
extern crate client_lib;
use dioxus_desktop::{Config, WindowBuilder};
use client_lib::USERNAME;


// 聊天客户端
fn main() {
    let username = std::env::args().nth(1).unwrap_or_else(|| "unknown".to_string());
    unsafe {
        USERNAME = username.clone();
    }
    let win_title = format!("{}的聊天窗口", username);

    dioxus_desktop::launch_cfg(
        client_lib::Client,
        Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(440.0, 450.0),
        ).with_title(win_title)),
    );
}