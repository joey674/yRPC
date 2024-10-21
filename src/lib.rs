pub mod rpc_provider;
pub mod rpc_application;
pub mod rpc_config;
pub mod rpc_protocol;


use serde_json::Value;


pub type RpcMethod = Box<dyn Fn(Value) -> Value + Send + Sync>;


pub trait Service: Send + Sync + 'static
{
    fn get_service_name(&self) -> &'static str;
    fn get_methods(&self) -> Vec<(&'static str, RpcMethod)>;
}

