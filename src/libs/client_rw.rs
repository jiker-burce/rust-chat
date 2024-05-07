use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
        println!("run");

        let addr = "127.0.0.1:6142".to_string();
        let stream = TcpStream::connect(addr).await.unwrap();
        let stream = Arc::new(Mutex::new(stream));


        // TODO 将服务器发送过来的数据写入到聊天窗口
        // 将服务器发送的数据写入「收取消息通道」，在消息对话框中启动一个线程，随时从此通道获取数据，并渲染UI
        let receive_task = {
            let mut stream = Arc::clone(&stream);
            let send_to_ui = receive_tx.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0; 1024];
                let mut stream = stream.lock().await;
                loop {
                    let n = stream.read(&mut buffer).await.expect("读取消息失败");
                    if n == 0 {
                        // 连接已关闭
                        break;
                    }
                    let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                    println!("server message: {}", message);
                    send_to_ui.send(message).await.expect("接收消息失败");
                }
            })
        };

        // TODO 将消息发送给服务器
        // 启动发送消息任务
        let send_task = {
            let stream = Arc::clone(&stream);
            tokio::spawn(async move {
                while let Some(message) = send_rx.recv().await {
                    let mut stream = stream.lock().await;
                    stream.write_all(message.as_bytes()).await.expect("Failed to send message to the server");
                }
            })
        };
        match tokio::try_join!(receive_task, send_task) {
            Ok(val) => {
                println!("{}", "cool");
            }
            Err(err) => {
                println!("Failed with {}.", err);
            }
        }
        println!("client_rw done");
    }
}