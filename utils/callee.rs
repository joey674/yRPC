use rpc_framework::{*,rpc_application::*, rpc_provider::*};
use serde::{Deserialize, Serialize};


/// 这里定义了两个req和res
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

/// 这里具体实现服务 并且这里的服务要满足框架提供的Service trait
/// 
/// 
/// 

pub struct UserService;

/// 业务层代码
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
/// 中间层
/// 这里是业务层代码和框架层代码的桥梁 需要进行转换
fn login_rpc(request: serde_json::Value) -> serde_json::Value
{   
    // 将 JSON 数据解析为 LoginRequest 结构体
    let request: LoginRequest = serde_json::from_value(request).unwrap();

    // 调用原始的 login_rpc 函数
    let response = login(request);

    // 将响应转换为 JSON
    serde_json::to_value(response).unwrap()           
}

fn logout_rpc(request: serde_json::Value) -> serde_json::Value
{
        // 将 JSON 数据解析为 LoginRequest 结构体
        let request: LogoutRequest = serde_json::from_value(request).unwrap();

        // 调用原始的 login_rpc 函数
        let response = logout(request);

        // 将响应转换为 JSON
        serde_json::to_value(response).unwrap()          
}



/// 实现Service trait, 这里更新一下 就是把写好的函数注册到sevice里

impl Service for UserService
{
    // 返回服务名称
    fn get_service_name(&self) -> &'static str 
    {
        "UserService"
    }

    // 返回方法名和方法映射
    fn get_methods(&self) -> Vec<(&'static str, RpcMethod)> 
    {   
        vec![
            (
                "login",
                Box::new(move |request: serde_json::Value| {
                    login_rpc(request)
                }),
            ),
            (
                "logout",
                Box::new(move |request: serde_json::Value| {
                    logout_rpc(request)
                }),
            ),
        ]
    }
}


/// 标准的启动过程
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



// pub struct GroupService;
// 
// impl Service for GroupService
// {
//     // 返回服务名称
//     fn get_service_name(&self) -> &'static str 
//     {
//         "GroupService"
//     }
// 
//     // 返回方法名和方法映射
//     fn get_methods(&self) -> Vec<(&'static str, Box<dyn Any + Send + Sync>)> 
//     {   
//         vec![]
//     }
// }
