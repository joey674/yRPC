use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};



async fn send_test_message(mut stream: TcpStream, json_data: &serde_json::Value)  
{   
    

}

#[tokio::main]
async fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    //     println!("rpc server listening on port 8080");

    //     loop {
    //         match listener.accept().await {
    //             Ok((socket, addr)) => {
    //                 println!("New connection from node: {}", addr);
    //                 // 连接上别的rpc节点；开启一个新的线程循环处理消息
    //                 let service_map = self.service_map.clone();
    //                 tokio::spawn(async move {
    //                     on_message(service_map,socket).await;
    //                 });                    
    //             }
    //             Err(e) => {
    //                 println!("Failed to accept connection: {}", e);
    //             }
    //         }
    //     }
}