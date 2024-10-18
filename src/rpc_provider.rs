use core::panic;
use std::collections::HashMap;
use std::ops::DerefMut;
use std::{any::Any};
use serde::{de::DeserializeOwned, Serialize,Deserialize};
use serde_json::Value;
use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt}; 
use std::sync::Arc;

use crate::rpc_protocol::*;
use super::*;


/// rpc服务提供者 算是一个runtime
/// 
/// 
pub struct RpcProvider
{
    service_map: Arc<std::collections::HashMap<String, ServiceInfo>>,
}

pub struct ServiceInfo 
{
    service: Box<dyn Service>,
    method_map: std::collections::HashMap<String, RpcMethod>,
}

impl  RpcProvider
{
    pub fn init(service_list: Vec<Box<dyn Service>>) -> Self 
    {   
        let mut service_map: std::collections::HashMap<String, ServiceInfo> = std::collections::HashMap::new();

        for service in service_list {
            let service_name = service.get_service_name();
            let methods = service.get_methods();
    
            let mut method_map: std::collections::HashMap<String, RpcMethod> = std::collections::HashMap::new();
            for (method_name, method) in methods {
                method_map.insert(method_name.to_string(), method);
            }
    
            let service_info = ServiceInfo {
                service: service,
                method_map: method_map,
            };
            service_map.insert(service_name.to_string(), service_info);
        }
        RpcProvider { service_map:Arc::new(service_map) }
    }

    pub async fn run(&self) 
    {   
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        println!("rpc server listening on port 8080");

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("New connection from node: {}", addr);
                    // 连接上别的rpc节点；开启一个新的线程循环处理消息
                    let service_map = self.service_map.clone();
                    tokio::spawn(async move {
                        on_message(service_map,socket).await;
                    });                    
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

/// 先假定用户的方法是同步的,用户的方法会分配到单独的线程上去执行
pub(crate) fn call_method( service_map: Arc<std::collections::HashMap<String, ServiceInfo>>,
                                service_name:String, 
                                method_name:String, 
                                args: Value) 
                                -> Value
{
    let method = service_map
        .get(&service_name).unwrap()
        .method_map.get(&method_name).unwrap();

    (method)(args)
}


async fn on_message(service_map: Arc<std::collections::HashMap<String, ServiceInfo>>, mut socket: TcpStream) 
{
    loop {
        // 读取/解析消息头 消息头是没有字节对齐的结构
        let mut header: [u8; RPC_MESSAGE_HEADER_LEN] = [0; RPC_MESSAGE_HEADER_LEN];
        if socket.read_exact(&mut header).await.is_err() {
            println!("Connection closed or failed to read message header.");
            break; 
        }
        let header = RpcMessageHeader::from_bytes(&header);
        let body_length = header.body_length;
        let _message_type = header.message_type;

        // 读取/解析消息体 发送端将用json格式序列化 所以这里我们可以直接用json反序列化
        let mut message_body = vec![0; body_length as usize];
        if socket.read_exact(&mut message_body).await.is_err() {
            println!("Failed to read the complete message body.");
            break;
        }
        let message_body = RpcMessageBody::from_bytes(&message_body);
        let service_name = message_body.service_name;
        let method_name = message_body.method_name;
        let args = message_body.args;
        
        let service_map = Arc::clone(&service_map);
        tokio::task::spawn_blocking( move || {
            call_method(service_map, service_name, method_name, args);
        });
    }
}

















