extern crate client_lib;

use dioxus_desktop::{Config, WindowBuilder};
use client_lib::USERNAME;

use dioxus::prelude::*;


// 聊天客户端
fn main() {
    // 需要 在toml中增加：dioxus-hot-reload = { version = "0.4.3", features = ["file_watcher", "dioxus-html"] }，
    // 否则会报： use of undeclared crate or module dioxus_hot_reload
    // 程序逻辑变化，自动重启：.with_rebuild_command("cargo run --bin client Bruce")
    // 目前虽然可以调通live reload，但是 无法达到修改css文件后自动重新渲染样式，还是需要手动重新rebuild
    // 已经提出issue：https://github.com/DioxusLabs/dioxus/issues/2379
    hot_reload_init!(dioxus_hot_reload::Config::new().with_paths(&["statics"]).with_rebuild_command("cargo run --bin client Bruce"));

    let username = std::env::args().nth(1).unwrap_or_else(|| "unknown".to_string());
    unsafe {
        USERNAME = username.clone();
    }

    let win_title = format!("{}的聊天窗口!", username);

    launch_desktop(
        client_lib::Client,
    );
    // Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
    //     dioxus_desktop::wry::application::dpi::LogicalSize::new(440.0, 450.0),
    // ).with_title(win_title)),
}