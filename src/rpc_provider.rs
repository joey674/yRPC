use tokio::net::{TcpListener,TcpStream};
use tokio::io::{AsyncReadExt,AsyncWriteExt}; 
use std::sync::Arc;

use crate::rpc_protocol::*;
use super::*;


static RPC_BUFFER_SIZE_PER_CONNECTION: usize = 1024 * 1024;


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
        println!("rpc provider listening on port 8080");

        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    println!("New connection from node: {}", addr);
                    // 连接上别的rpc节点；开启一个新的线程循环处理消息
                    let service_map = self.service_map.clone();
                    tokio::spawn(async move {
                        on_connect(service_map,socket).await;
                    });                    
                }
                Err(e) => {
                    println!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

/// 这里的场景是一个rpc节点连接上了另一个rpc节点，这个节点在一次连接中会发来多条请求； 
/// 我们在这里希望并行地完成这些请求；
async fn on_connect(service_map: Arc<std::collections::HashMap<String, ServiceInfo>>, mut socket: TcpStream) 
{   
    let mut handle_list = Vec::new();



    // 先一次性读取到buffer中
    let mut buffer:Vec<u8>  = vec![0; RPC_BUFFER_SIZE_PER_CONNECTION];
    let message_len = match socket.read(&mut buffer).await {
        Ok(len) if len > 0 => len,
        _ => {
            println!("Connection closed or failed to read.");
            return;
        }
    };
    buffer.truncate(message_len);

    // 对buffer进行解析
    let mut cursor = std::io::Cursor::new(&buffer);
    loop{
        // 读取/解析消息头 消息头是没有字节对齐的结构
        let mut header: [u8; RPC_MESSAGE_HEADER_LEN] = [0; RPC_MESSAGE_HEADER_LEN];
        if std::io::Read::read_exact(&mut cursor, &mut header).is_err(){
            break;
        }
        let header = RpcMessageHeader::from_bytes(&header);

        // 读取消息体 发送端将用json格式序列化 所以这里我们可以直接用json反序列化
        let mut body = vec![0; header.body_length as usize];
        if std::io::Read::read_exact(&mut cursor, &mut body).is_err() {
            println!("Failed to read the complete message body.");
            break;
        }
        let message_body = RpcMessageBody::from_bytes(&body);

        // 这里先用spawn_blocking开启多线程去执行用户的方法；这里默认用户的方法是计算密集型的； 
        let service_map = Arc::clone(&service_map);

        let handle = tokio::task::spawn_blocking( move || {
            (call_method(service_map, message_body),header.message_id)
        });
        handle_list.push(handle);
    }

    // 等待所有的方法的执行线程结束
    for handle in handle_list {
        if let Ok((responsebody, uuid)) = handle.await {
            let responsebody = responsebody.to_bytes();
            let responseheader = RpcMessageHeader {
                body_length: responsebody.len() as u32,
                message_id: uuid,
            };

            let responseheader = responseheader.to_bytes();
            let message: Vec<u8> = [responseheader, responsebody].concat();
            
            if socket.write_all(&message).await.is_err() {
                println!("Failed to write response message");
            }
        }
    }

    // 关闭连接; 一次socket连接中可能有多个请求，但是处理完就关闭连接
    let _ = socket.shutdown().await;
}


/// 先假定用户的方法是同步的,用户的方法会分配到单独的线程上去执行
pub(crate) fn call_method( 
    service_map: Arc<std::collections::HashMap<String, ServiceInfo>>, req_message_body: RpcMessageBody) 
    -> RpcMessageBody
{   
    let service_name = req_message_body.service_name;
    let method_name = req_message_body.method_name;
    let args = req_message_body.args;

    let method = service_map
    .get(&service_name).unwrap()
    .method_map.get(&method_name).unwrap();

    let res = (method)(args);
    RpcMessageBody {
        message_type: MessageType::Response,
        service_name: service_name,
        method_name: method_name,
        args: res,
    }
}












