
use serde::{Deserialize, Serialize};

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


#[derive(Serialize, Deserialize, Debug)]
pub struct GetGroupsRequest 
{
    pub username: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GetGroupsResponse 
{
    pub group_list: Vec<String>,
    pub success: bool,
}



#[allow(dead_code)]
fn main() {}
