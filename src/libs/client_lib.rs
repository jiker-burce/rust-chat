#![allow(non_snake_case)]

use dioxus::prelude::*;

pub static mut USERNAME: String = String::new();

#[derive(PartialEq, Clone)]
struct Message {
    id: usize,
    user: String,
    message: String
}

// 寻找参考方法：dioxuslabs官网 Learn -》Dynamic Rendering =》Edit this page =》docsite/docs-src/0.5/en =》reference =》dynamic_rendering.md =》code
//    找到第77行，发现：{{#include src/doc_examples/rendering_lists.rs:render_list}}，
//    于是找到当前仓库对应目录，发现参考源代码：https://github.com/DioxusLabs/docsite/blob/main/src/doc_examples/rendering_lists.rs
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

    let mut msg_field = use_signal(String::new);

    // 状态
    let mut messages: Signal<Vec<Message>> = use_signal(Vec::new);
    // messages.write().push(Message{id: 0, user: current_user.clone(), message: "hello1".to_string()});
    // messages.write().push(Message{id: 1,user: current_user.clone(), message: "hello2".to_string()});

    let mut next_id = use_signal(|| 0);

    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" },
            link { href:"../../statics/client_style.css", rel:"stylesheet"},
            form {
                onsubmit: move |event| {
                    let current_user = unsafe{
                        USERNAME.clone()
                    };

                    messages
                        .write()
                        .push(Message {id: next_id(), user: current_user, message: msg_field()});

                    next_id += 1;

                    msg_field.set(String::new());
                },
                div {
                    class: "chat-container",
                    div {
                        class: "chat-messages",
                        for msg in messages() {
                            MessageEntry {msg}
                        }
                    }
                    div {
                            class: "chat-input",
                            input {
                                r#type: "text",
                                placeholder: "Type your message...",
                                value: "{msg_field}",
                                oninput: move |event| msg_field.set(event.value())
                            },
                            button {
                                r#type: "submit",
                                "发送"
                            }
                        }
                }
            }
        }
    }
}
