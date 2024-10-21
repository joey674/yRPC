pub mod rpc_provider;
pub mod rpc_application;
pub mod rpc_config;
pub mod rpc_protocol;


use serde_json::Value;

/// Service trait
/// 供callee调用 为自己的Service实现
/// 
/// 

pub type RpcMethod = Box<dyn Fn(Value) -> Value + Send + Sync>;

pub trait Service: Send + Sync + 'static
{
    fn get_service_name(&self) -> &'static str;
    fn get_methods(&self) -> Vec<(&'static str, RpcMethod)>;
}

