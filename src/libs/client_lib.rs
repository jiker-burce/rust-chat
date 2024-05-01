#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::html::{div, form, input, span};
use dioxus_desktop::{Config, WindowBuilder};

pub static mut USERNAME: String = String::new();

#[derive(PartialEq, Clone)]
struct Message {
    user: String,
    message: String
}

/**
注意：
1， 组件必须加上
    #[component] 和 cx: Scope
*/
#[component]
fn MessageEntry(msg: Message) -> Element {
    rsx! {
        div {
            class: "message",
            span {
                    class: "username",
                    "{msg.user}: "
                },
            span {
                    class: "text",
                    "{msg.message}"
            }
        }
    }
}

// 聊天客户端窗口
pub fn Client() -> Element {
    let current_user = unsafe{
        USERNAME.clone()
    };
    // 状态
    let mut messages: Signal<Vec<Message>> = use_signal(Vec::new);
    messages.push(Message{user: current_user.clone(), message: "hello1".to_string()});
    messages.push(Message{user: current_user.clone(), message: "hello2".to_string()});

    let messages_lock = messages.read();
    let messages_rendered = messages_lock.iter().map(|msg| {
        rsx! { MessageEntry { msg: msg.clone() } }
    });

    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" },
            link { href:"../../statics/client_style.css", rel:"stylesheet"},
            main {
                div {
                    class: "chat-container",
                    {messages_rendered},
                    div {
                            class: "chat-input",
                            input {
                                r#type: "text",
                                placeholder: "Type your message..."
                            },
                            button {
                                r#type: "submit",
                                onclick: move |event| {
                                    let current_user = unsafe{
                                        USERNAME.clone()
                                    };
                                    messages
                                    .write()
                                    .push(Message {user: current_user, message: "new message!".to_string()})
                                },
                                "发送"
                            }
                        }
                }
            }
        }
    }
}
