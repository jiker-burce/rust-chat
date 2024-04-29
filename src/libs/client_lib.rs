#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::str::FromStr;
use dioxus::html::{div, form, input, span};
use dioxus_desktop::{Config, WindowBuilder};

struct Message {
    user: String,
    message: String
}

// 计算器应用程序
pub fn Client(cx: Scope) -> Element {
    // 状态
    let messages = use_state(cx, || Vec::<Message>::new());

    render!(div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { include_str!("../../statics/client_style.css") }
        header {
            h1 { "客户端" }
        }
        main {
            rsx! {
            div {
                class: "chat-container",
                div {
                    class: "chat-messages",
                    div {
                        class: "message",
                        span {
                                class: "username",
                                "User1:"
                            },
                        span {
                                class: "text",
                                "Hello"
                            }
                    },
                    div {
                        class: "message",
                        span {
                                class: "username",
                                "User2:"
                            },
                        span {
                                class: "text",
                                "Hello2"
                            }
                    }
                },
                div {
                        class: "chat-input",
                        input {
                            r#type: "text",
                            placeholder: "Type your message..."
                        },
                        button {
                            r#type: "submit",
                            onclick: move |event| {
                                //
                            },
                            "发送"
                        }
                    }
            }
        }
    }})
}
