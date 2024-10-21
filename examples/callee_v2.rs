use rpc_framework::{*,rpc_application::*, rpc_provider::*};

#[path = "protocol.rs"]
mod protocol;
use protocol::*;


#[yrpc::service(login,logout)]
pub struct UserService;

#[yrpc::method]
fn login(request: LoginRequest) -> LoginResponse 
{
    println!("login method called. request: {:?}", request);
    LoginResponse {
        login_result: LoginResult {
            code: 11,
            message: 22,
            success: true,
        },
        success: true,
    }
}
#[yrpc::method]
fn logout(request: LogoutRequest) -> LogoutResponse 
{
    println!("logout method called. request: {:?}", request);
    LogoutResponse {
        logout_result: LogoutResult {
            code: 33,
            message: 44,
            success: true,
        },
        success: true,
    }
}


#[yrpc::service(get_groups)]
pub struct GroupService;
#[yrpc::method]
fn get_groups(request: GetGroupsRequest) -> GetGroupsResponse 
{
    println!("get_group method called. request: {:?}", request);
    GetGroupsResponse {
        group_list: vec!["group1".to_string(), "group2".to_string()],
        success: true,
    }
}


#[tokio::main]
async fn main() 
{      
    RpcApplication::init();

    let user_service = UserService;
    let group_service = GroupService;
    let services_list: Vec<Box<dyn Service>> = vec![
        Box::new(group_service),
        Box::new(user_service)
    ];
    let rpc_provider = RpcProvider::init(services_list);

    rpc_provider.run().await;
}