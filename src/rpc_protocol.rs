use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::mem;


/// 定义消息头结构体
/// message_type: 0表示请求 1表示响应
#[repr(packed)]
pub struct RpcMessageHeader {
    pub body_length: u32,
    pub message_id: uuid::Uuid,
}

pub const RPC_MESSAGE_HEADER_LEN: usize = mem::size_of::<RpcMessageHeader>(); 

impl RpcMessageHeader {
    /// 序列化/反序列化字节流 
    /// 这里不直接发送struct 因为struct会有内存对齐问题 所以手动转化
    /// 同时这里要考虑大端序小端序的问题 要从本地存储的小端序和网络传输的大端序进行转换
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(RPC_MESSAGE_HEADER_LEN); 
        buffer.extend(&self.body_length.to_be_bytes()); 
        buffer.extend(self.message_id.as_bytes());
        buffer
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let body_length = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let message_id = uuid::Uuid::from_slice(&bytes[4..20]).unwrap();

        RpcMessageHeader {
            body_length,
            message_id
        }
    }
}


/// 定义消息体结构体
/// 
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcMessageBody {
    pub message_type: MessageType,
    pub service_name: String,
    pub method_name: String,
    pub args: Value,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Request = 1,
    Response = 2,
}

impl RpcMessageBody {
    /// 序列化/反序列化字节流 
    /// 发送端将用json格式序列化 所以这里我们可以直接用json反序列化，不用考虑大端序小端序的问题
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap() 
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        serde_json::from_slice(&bytes).unwrap()
    }
}


#[test]
fn test()
{
    let header = RpcMessageHeader 
    {
        body_length: 100,
        message_id: uuid::Uuid::new_v4(),
    };
    let size = mem::size_of::<RpcMessageHeader>();
    println!("{:?}", size);
    println!("{:?}", RPC_MESSAGE_HEADER_LEN);
}