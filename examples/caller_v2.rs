use rpc_framework::rpc_application::*;
use rpc_framework::rpc_channel::*;
#[path = "protocol.rs"]
mod protocol;
use protocol::*;




#[tokio::main]
async fn main() {
    RpcApplication::init();
    
    let handles: Vec<_> = (0..10).map(|i| {
        let request = LoginRequest {
            username: format!("user{}", i),
            password: "123456".to_string(),
        }; 

        tokio::task::spawn(async move {
            let response = get_channel_instance()
                .await
                .send_request::<LoginRequest, LoginResponse>("UserService", "login", request)
                .await
                .unwrap();

            println!("Response from task {}: {:#?}", i, response);
        })
    }).collect();

    // 等待所有任务完成
    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {}", e);
        }
    }
}