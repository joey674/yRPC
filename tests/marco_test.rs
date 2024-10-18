use rpc_macros::RpcMacro;
use rpc_framework::TestService;

#[derive(RpcMacro)]
struct UserService;

#[derive(RpcMacro)]
struct GroupService;

fn main() {
    let user_service = UserService;
    let group_service = GroupService;
    println!("{}", user_service.get_service_name());
    println!("{}", group_service.get_service_name());
}
