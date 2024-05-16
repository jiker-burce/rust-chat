#![allow(non_snake_case)]


use dioxus::prelude::*;
use std::cell::RefCell;
use std::env;
use std::env::current_exe;
use std::sync::Arc;
use dioxus_desktop::use_window;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(PartialEq, Clone, Debug)]
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
    let (receive_tx, mut receive_rx) = mpsc::channel::<String>(100);

    spawn(async move {
        crate::ChatClient::new().run(receive_tx, send_rx).await;
    });

    let current_user = use_signal(|| String::new());
    let mut messages = use_signal_sync(|| {
        vec![]
    });

    // 接收消息并写入 messages
    spawn(async move {
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        println!("send msg");
        while let Some(message) = receive_rx.recv().await {
            messages.push(Message{user: current_user.to_string(), message },);
        }
    });

    // 判断current_message是否有新数据从页面发送，存在就发送数据给数据流
    let mut current_message: Signal<Option<Message>> = use_signal(|| None);
    spawn( async move {
        loop {
            if let Some(msg) = current_message.take() {
                send_tx.send(msg.message).await.expect("窗口发送数据失败");
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    rsx! {
        ClientForm{
            messages: messages,
            current_message: current_message,
            current_user: current_user
        }
    }
}

#[component]
fn ClientForm(
    messages: Signal<Vec<Message>, SyncStorage>,
    mut current_message: Signal<Option<Message>>,
    current_user: Signal<String>
) -> Element {
    let mut msg_field = use_signal(String::new);

    rsx! {
        div {
            link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" },
            link { href:"../../statics/client_style.css", rel:"stylesheet"},
            form {
                onsubmit: move |event| {
                    current_message.set(None);

                    let msg = msg_field();
                    let msg_str = msg.clone() + &"\n";
                    let window_handle = use_window();
                    if window_handle.title().eq("chat window") {
                        current_user.set(msg);
                        window_handle.set_title(&( current_user.to_string() + &"'s chat window"));
                    }
                    let message = Message {user: current_user.to_string(), message: msg_str};
                    messages.write().push(message.clone());

                    msg_field.set(String::new());
                    current_message.set(Some(message));
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
            // span {
            //         class: "username",
            //         "{msg.user}: "
            //     },
            span {
                    class: "text",
                    "{msg.message}"
            }
        }
    }
}
