use serde::{Deserialize, Serialize};
use rpc_framework::{*,rpc_application::*, rpc_provider::*};
use rpc_macros::*;

///
/// 
/// 
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest 
{
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResult 
{
    pub code : u32,
    pub message: u8,
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse 
{
    pub login_result: LoginResult,
    pub success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutRequest 
{
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutResult 
{
    pub code : u32,
    pub message: u8,
    pub success: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutResponse 
{
    pub logout_result: LogoutResult,
    pub success: bool,
}

/// 
/// 
/// 
#[rpc_method]
fn login(request: LoginRequest) -> LoginResponse 
{
    println!("login method called. request: {:?}", request);
    LoginResponse {
        login_result: LoginResult {
            code: 0,
            message: 0,
            success: true,
        },
        success: true,
    }
}

#[rpc_method]
fn logout(request: LogoutRequest) -> LogoutResponse 
{
    println!("logout method called. request: {:?}", request);
    LogoutResponse {
        logout_result: LogoutResult {
            code: 0,
            message: 0,
            success: true,
        },
        success: true,
    }
}

///  这个派生类会把UserService实现的方法注册到rpc_provider中
/// 
/// 
#[rpc_service(login,logout)]
pub struct UserService;


#[tokio::main]
async fn main() 
{      
    RpcApplication::init();

    let user_service = UserService;
    let services_list: Vec<Box<dyn Service>> = vec![
        Box::new(user_service)
    ];
    let mut rpc_provider = RpcProvider::init(services_list);

    rpc_provider.run().await;

    
}