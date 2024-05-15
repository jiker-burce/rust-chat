use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

pub struct ChatClient {
    // stream: TcpStream,
    // receive_tx: Option<Sender<String>>,
    // send_rx: Option<Receiver<String>>
}

impl ChatClient {
    pub fn new() -> Self {
        ChatClient{
            // receive_tx: Some(receive_tx), send_rx: Some(send_rx)
        }
    }

    pub async fn run(&mut self, receive_tx: Sender<String>, mut send_rx: Receiver<String>) {
        let addr = "127.0.0.1:6142".to_string();
        let stream = TcpStream::connect(addr).await.unwrap();
        let (read, write) = stream.into_split();

        // 分离读写结构，防止死锁
        let reader = Arc::new(Mutex::new(read));
        let writer = Arc::new(Mutex::new(write));

        let receive_task = {
            let reader = Arc::clone(&reader);
            let send_to_ui = receive_tx.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                loop {
                    let n = reader.lock().await.read(&mut buffer).await.expect("读取消息失败");
                    println!("get r lock");
                    if n == 0 {
                        break; // 连接已关闭
                    }
                    let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                    send_to_ui.send(message).await.expect("接收消息失败");
                }
            })
        };

        let send_task = {
            let writer = Arc::clone(&writer);
            tokio::spawn(async move {
                while let Some(message) = send_rx.recv().await {
                    let mut w = writer.lock().await;
                    w.write_all((message).as_bytes()).await.expect("Failed to send message to the server");
                }
            })
        };

        match tokio::try_join!(receive_task, send_task) {
            Ok(_) => println!("All tasks completed successfully."),
            Err(e) => println!("A task failed with error: {}", e),
        }
    }
}
