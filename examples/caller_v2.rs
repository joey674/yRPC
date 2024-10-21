use core::time;

use rpc_framework::rpc_protocol::*;
use tokio::net;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;

#[path = "protocol.rs"]
mod protocol;
use protocol::*;


pub fn create_message() -> Vec<u8> 
{
    let request = LoginRequest {
        username: "dd".to_string(),
        password: "123456".to_string(),
    };
    let messagebody = RpcMessageBody {
        message_type: MessageType::Request,
        service_name: "UserService".to_string(),
        method_name: "login".to_string(),
        args: serde_json::to_value(request).unwrap(),
    };
    let messagebody = messagebody.to_bytes();

    let messageheader = RpcMessageHeader {
        body_length: messagebody.len() as u32,
        message_id: uuid::Uuid::new_v4(),
    };
    let messageheader = messageheader.to_bytes();
    let message_1 = [messageheader, messagebody].concat();

    
    let request = LogoutRequest {
        username: "qq".to_string(),
    };
    let messagebody = RpcMessageBody {
        message_type: MessageType::Request,
        service_name: "UserService".to_string(),
        method_name: "logout".to_string(),
        args: serde_json::to_value(request).unwrap(),
    };
    let messagebody = messagebody.to_bytes();

    let messageheader = RpcMessageHeader {
        body_length: messagebody.len() as u32,
        message_id: uuid::Uuid::new_v4(),
    };
    let messageheader = messageheader.to_bytes();
    let message_2 = [messageheader, messagebody].concat();


    let request = GetGroupsRequest {
        username: "zz".to_string(),
    };
    let messagebody = RpcMessageBody {
        message_type: MessageType::Request,
        service_name: "GroupService".to_string(),
        method_name: "get_groups".to_string(),
        args: serde_json::to_value(request).unwrap(),
    };
    let messagebody = messagebody.to_bytes();

    let messageheader = RpcMessageHeader {
        body_length: messagebody.len() as u32,
        message_id: uuid::Uuid::new_v4(),
    };
    let messageheader = messageheader.to_bytes();
    let message_3 = [messageheader, messagebody].concat();


    [message_1, message_2,message_3].concat()
}



#[tokio::main]
async fn main() {
    if let Ok(mut socket) =  net::TcpStream::connect("127.0.0.1:8080").await {

        let message = create_message();
        if let Err(e) = socket.write_all(&message).await {
            println!("Failed to send message: {}", e);
        } else {
            println!("Message sent successfully.");
        }

        loop{
            let mut header: [u8; RPC_MESSAGE_HEADER_LEN] = [0; RPC_MESSAGE_HEADER_LEN];
            if socket.read_exact(&mut header).await.is_err() {
                println!("Connection closed.");
                socket.shutdown().await.unwrap();
                break;  
            }
            let header = RpcMessageHeader::from_bytes(&header);

            let mut message_body = vec![0; header.body_length as usize];
            socket.read_exact(&mut message_body).await.expect("Failed to read the complete message body.");
            let message_body = RpcMessageBody::from_bytes(&message_body);
            println!("message_body: {:#?}", message_body);
        }
    }

    tokio::time::sleep(time::Duration::from_secs(1)).await;
    println!("-------------------");

    if let Ok(mut socket) =  net::TcpStream::connect("127.0.0.1:8080").await {

        let message = create_message();
        if let Err(e) = socket.write_all(&message).await {
            println!("Failed to send message: {}", e);
        } else {
            println!("Message sent successfully.");
        }

        loop{
            let mut header: [u8; RPC_MESSAGE_HEADER_LEN] = [0; RPC_MESSAGE_HEADER_LEN];
            if socket.read_exact(&mut header).await.is_err() {
                println!("Connection closed.");
                socket.shutdown().await.unwrap();
                break;  
            }
            let header = RpcMessageHeader::from_bytes(&header);

            let mut message_body = vec![0; header.body_length as usize];
            socket.read_exact(&mut message_body).await.expect("Failed to read the complete message body.");
            let message_body = RpcMessageBody::from_bytes(&message_body);
            println!("message_body: {:#?}", message_body);
        }
    }


}