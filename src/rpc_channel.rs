use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use tokio::net::TcpStream;
use crate::rpc_protocol::*; 
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;


pub struct RpcChannel {
    pub server_address: String,
}

impl RpcChannel {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(RpcChannel {
            server_address: "127.0.0.1:8080".to_string(),
        })
    }

    pub async fn send_request<T, D>(&self, service_name: &str, method_name: &str, args: T) 
        -> Result<D, Box<dyn std::error::Error>> 
    where
    T: Serialize,
    D: for<'a> Deserialize<'a>,
    {
        if let Ok(mut socket) =  TcpStream::connect(self.server_address.clone()).await {

            let message_body = RpcMessageBody {
                message_type: MessageType::Request,
                service_name: service_name.to_string(),
                method_name: method_name.to_string(),
                args: serde_json::to_value(args).unwrap(),
            };
            let message_body = message_body.to_bytes();
            
            let message_id = uuid::Uuid::new_v4();
            let message_header = RpcMessageHeader {
                body_length: message_body.len() as u32,
                message_id: message_id.clone(),
            };
            let message_header = message_header.to_bytes();
            let message = [message_header, message_body].concat();


            if let Err(e) = socket.write_all(&message).await {
                return Err("Failed to send message".into()) 
            } 

            let mut header: [u8; RPC_MESSAGE_HEADER_LEN] = [0; RPC_MESSAGE_HEADER_LEN];
            if socket.read_exact(&mut header).await.is_err() {
                socket.shutdown().await.unwrap();
                return Err("An error occurred".into()) 
            }
            let header = RpcMessageHeader::from_bytes(&header);
            // check
            // if header.message_id != message_id {
            //     return Err("Message id not match, this is wrong message".into())
            // }
            // if message_body.message_type == MessageType::Response {
            //     return Err("Message id not match, this is wrong message".into())
            // }

            let mut message_body = vec![0; header.body_length as usize];
            socket.read_exact(&mut message_body).await.expect("Failed to read the complete message body.");
            let message_body = RpcMessageBody::from_bytes(&message_body);

            let res: D = serde_json::from_value(message_body.args).unwrap();

            return Ok(res)
        }

        Err("An error occurred".into())
    }
}

static RPC_CHANNEL: OnceCell<RpcChannel> = OnceCell::const_new();
pub async fn get_channel_instance() -> &'static RpcChannel {
    RPC_CHANNEL
        .get_or_init(|| async { RpcChannel::new().unwrap() })
        .await
}

