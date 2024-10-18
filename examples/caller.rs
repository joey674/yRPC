use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};




// // 发送带有自定义消息头的 JSON 数据
// async fn send_custom_message(mut stream: TcpStream, json_data: &serde_json::Value) -> tokio::io::Result<()> {
//     // 序列化 JSON 数据
//     let json_string = serde_json::to_string(json_data).unwrap();
//     let length = json_string.len() as u32;

//     // 生成消息头
//     let header = MessageHeader {
//         length,
//         message_type: 1, // 假设 1 表示请求消息
//         message_id: 12345678, // 假设是消息的唯一 ID
//         timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
//     };

//     // 发送消息头（21字节：4字节长度，1字节类型，8字节ID，8字节时间戳）
//     stream.write_u32(header.length).await?;
//     stream.write_u8(header.message_type).await?;
//     stream.write_u64(header.message_id).await?;
//     stream.write_u64(header.timestamp).await?;

//     // 发送消息体（JSON 数据）
//     stream.write_all(json_string.as_bytes()).await?;

//     Ok(())
// }


fn main() {
    println!("Hello, world!");
}