extern crate client_mod;

use client_mod::client_lib::USERNAME;

use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use dioxus_desktop::tao;

// 聊天客户端
// #[tokio::main]
fn main(){
    // 需要 在toml中增加：dioxus-hot-reload = { version = "0.4.3", features = ["file_watcher", "dioxus-html"] }，
    // 否则会报： use of undeclared crate or module dioxus_hot_reload
    // 程序逻辑变化，自动重启：.with_rebuild_command("cargo run --bin client Bruce")
    // 目前虽然可以调通live reload，但是 无法达到修改css文件后自动重新渲染样式，还是需要手动重新rebuild
    // 已经提出issue：https://github.com/DioxusLabs/dioxus/issues/2379
    // hot_reload_init!(dioxus_hot_reload::Config::new().with_paths(&["statics"]));//.with_rebuild_command("cargo run --bin client Bruce"));

    let username = std::env::args().nth(1).unwrap_or_else(|| "unknown".to_string());
    unsafe {
        USERNAME = username.clone();
    }

    let win_title = format!("{}的聊天窗口!", username);

    // 无参数启动
    // launch_desktop(
    //     client_lib::Client,
    // );

    // 启动方法参考(注意参考源码时，一定要找Cargo.toml里面引用的dioxus是0.5.0的，也就是和当前Cargo.toml中的版本保持一致，否则很有可能导致代码无法编译通过，
    //  因为0.4.0 的代码风格，渲染宏函数和0.5.0完全不一样了。)：
    // https://github.com/DioxusLabs/dioxus/blob/main/examples/file_explorer.rs
    // 参数使用tao原生类型；dioxus 的桌面渲染基本采用tao的引擎了： https://dioxuslabs.com/learn/0.5/reference/desktop
    LaunchBuilder::desktop().with_cfg(
        Config::new().with_window(
            WindowBuilder::new()
                .with_resizable(false)
                .with_inner_size(tao::dpi::LogicalSize::new(340.0, 450.0))
                .with_title(win_title)
        )
    )
    .launch(client_mod::client_lib::Client);
}