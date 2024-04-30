#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::html::{div, form, input, span};
use dioxus_desktop::{Config, WindowBuilder};

pub static mut USERNAME: String = String::new();

#[derive(PartialEq)]
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
fn MessageEntry(cx: Scope, list: Vec<Message>) -> Element {
    render! {
        div {
            class: "chat-messages",
            key: "1",
            {list.iter().map(|item| rsx!(
                div {
                    class: "message",
                    span {
                            class: "username",
                            "{item.user}: "
                        },
                    span {
                            class: "text",
                            "{item.message}"
                        }
                }
            ))}
        }
    }
}

// 计算器应用程序
pub fn Client(cx: Scope) -> Element {
    let current_user = unsafe{
        USERNAME.clone()
    };
    // 状态
    let messages = use_state(cx, || Vec::<Message>::new());
    let messages = vec![
        Message{user: current_user.clone(), message: "hello1".to_string()},
        Message{user: current_user.clone(), message: "hello2".to_string()},
    ];

    render!(div {
        link { href:"https://fonts.googleapis.com/icon?family=Material+Icons", rel:"stylesheet" }
        style { include_str!("../../statics/client_style.css") }
        main {
            rsx! {
            div {
                class: "chat-container",
                MessageEntry{list: messages},
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
