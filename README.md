# yRPC
A lightweight RPC framework for Rust with a focus on ease of use.


# Usage

Add to your Cargo.toml dependencies:
```
```


protocol.rs:
The caller and callee should follow the same Interface protocol. In yRPC, a protocol.rs file is used; it plays a role like .proto file. But we dont define our services here.
```rust
// protocol.rs
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
```


callee.rs:
We start first with the callee part. the service should define like:
```rust
// callee.rs
#[yrpc::service(login)]
pub struct UserService;
```
the methods can be multiple: #[yrpc::service(login,logout)];

then we implement the methods:
```rust
// callee.rs
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

```

after that, we write following code in main(the framework is based on tokio) to register the service to the provider:
```rust
#[tokio::main]
async fn main() 
{      
    RpcApplication::init();

    let services_list: Vec<Box<dyn Service>> = vec![
        Box::new(GroupService),
        Box::new(UserService)
    ];
    let rpc_provider = RpcProvider::init(services_list);

    rpc_provider.run().await;
}
```


For the caller part:
```rust
// caller.rs
#[tokio::main]
async fn main() {
    RpcApplication::init();

    let request = LoginRequest {
        username: "dd".to_string(),
        password: "123456".to_string(),
    };

    let responce = get_channel_instance()
        .await
        .send_request::<LoginRequest,LoginResponse>("UserService", "login", request)
        .await
        .unwrap();

    println!("responce: {:#?}", responce);
}
```
that is it! the rpc service is running and caller can use method provided by server!