
extern crate client_lib;
use dioxus_desktop::{Config, WindowBuilder};

// 计算器应用程序
fn main() {
    dioxus_desktop::launch_cfg(
        client_lib::Client,
        Config::default().with_window(WindowBuilder::new().with_resizable(false).with_inner_size(
            dioxus_desktop::wry::application::dpi::LogicalSize::new(440.0, 450.0),
        )),
    );
}