#![allow(non_snake_case)]


use dioxus::prelude::*;
use std::cell::RefCell;
use std::env;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

pub static mut USERNAME: String = String::new();

// struct ChannelRS {
//     receive_rx: Receiver<String>,
//     send_tx: Sender<String>
// }

#[derive(PartialEq, Clone)]
struct Message {
    user: String,
    message: String
}

// 聊天客户端窗口
pub fn Client() -> Element {
    println!("Client");
    // 创建发送消息的通道
    let (send_tx, mut send_rx) = mpsc::channel::<String>(100);
    // 创建接收消息的通道
    let (receive_tx, receive_rx) = mpsc::channel::<String>(100);

    spawn(async move {
        crate::ChatClient::new().run(receive_tx, send_rx).await;
    });

    let recv = Signal::new(receive_rx);
    let send = Signal::new(send_tx);

    rsx! {
        ClientForm{receive_rx: recv, send_tx: send}
    }
}

#[component]
fn ClientForm(mut receive_rx: Signal<Receiver<String>>, mut send_tx: Signal<Sender<String>>) -> Element {
    let current_user = unsafe{ USERNAME.clone() };
    let mut msg_field = use_signal(String::new);


    let mut messages = use_signal_sync(|| {
        vec![
            Message{user: current_user.clone(), message: "hello1".to_string()},
            Message{user: current_user.clone(), message: "hello2".to_string()},
        ]
    });
    spawn(async move {
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let mut recv = receive_rx.take();
        while let Some(message) = recv.recv().await {
            messages.push(Message{user: current_user.clone(), message },);
        }
    });

    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" },
            link { href:"../../statics/client_style.css", rel:"stylesheet"},
            form {
                onsubmit: move |event| {
                    let current_user = unsafe{USERNAME.clone()};

                    messages
                        .write()
                        .push(Message {user: current_user, message: msg_field()});

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

// 寻找参考方法：dioxuslabs官网 Learn -》Dynamic Rendering =》Edit this page =》docsite/docs-src/0.5/en =》reference =》dynamic_rendering.md =》code
//    找到第77行，发现：{{#include src/doc_examples/rendering_lists.rs:render_list}}，
//    于是找到当前仓库对应目录，发现参考源代码：https://github.com/DioxusLabs/docsite/blob/main/src/doc_examples/rendering_lists.rs
// TODO
// 1，不同的颜色区分本地发送的数据和接收的数据
// 2，接收的数据左对齐呈现，自己发送的数据右对齐
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
