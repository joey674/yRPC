/// 宏分为两种类型：声明宏和过程宏；过程宏分为三种类型：函数宏、属性宏和派生宏；
/// 其中声明宏和函数宏不常用 重点关注下面两个宏：
/// 属性宏	修饰函数、结构体、枚举等元素，生成代码	#[my_attribute]	#[proc_macro_attribute]
/// 派生宏	自动为结构体或枚举实现特定的 trait	#[derive(MyTrait)]	#[proc_macro_derive]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::DeriveInput;
use proc_macro::Span;

/// 示例1  派生宏
/// 本示例只用作演示
/// 
/// 结构体类型
/// 
/// vis，可视范围               ident，标识符     generic，范型    
/// pub             struct      User            <'a, T>          
/// fields: 结构体的字段
/// {
///     // vis   ident   type
///        pub   name:   &'a T, 
/// }
#[proc_macro_derive(RpcMacro)]
pub fn rpc_macro_derive(input: TokenStream) -> TokenStream {
    // 这里的源码（比如一个结构体）解析出来
    let ast:DeriveInput = syn::parse(input).unwrap();

    // 构建特征实现代码
    impl_rpc_macro(&ast)
}
fn impl_rpc_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl TestService for #name {
            // fn get_service_name(&self) -> &'static str;
            fn get_service_name(&self) -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}


/// 示例2 属性宏 
/// 本实例将用于库内； 所做的事情是把用户自定的业务生成中间层函数供rpc框架调用
/// 
/// 第一个参数是属性宏中传入的参数 第二个参数是被标注的函数信息
/// 
#[proc_macro_attribute]
pub fn rpc_method(_: TokenStream, item: TokenStream) -> TokenStream 
{
    // 解析标注的函数
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_return_type = &input_fn.sig.output;

    // 解析函数的参数
    // syn::FnArg 枚举有两种变体，用于表示函数的不同类型的参数：
    // syn::FnArg::Typed：表示类型化的参数，形如 arg: MyType 或 param: i32。这表示一个有名字和类型的参数。
    // syn::FnArg::Receiver：表示 self 或 &self，即方法的接收者。在方法（impl 块中的函数）中，self 是第一个参数。
    // 这里我们就取出第一个参数的类型；
    let arg_type = if let Some(syn::FnArg::Typed(arg)) = fn_args.first() {
        &arg.ty
    } else {
        panic!("rpc_method must have at least one argument");
    };

    // 将方法名保留给中间层函数，原始函数重命名
    let fn_name_origin = syn::Ident::new(&format!("{}_origin", fn_name), fn_name.span());

    // 生成新的代码，包括中间层和原始函数的包装
    let expanded = quote! {
        // 原始的业务方法
        pub fn #fn_name_origin(#fn_args) #fn_return_type {
            #fn_block
        }

        // 自动生成的中间层方法
        pub fn #fn_name(request: serde_json::Value) -> serde_json::Value {
            let request: #arg_type = serde_json::from_value(request).unwrap();
            let response = #fn_name_origin(request);
            serde_json::to_value(response).unwrap()
        }
    };
    TokenStream::from(expanded)
}


/// 示例3 属性宏
/// 本实例将用于库内； 所做的事情是把用户自定的方法注册给对应的服务，其实就是实现Service trait
/// 这里不用派生宏的原因是 虽然我们是为了实现一个trait， 但是我想在实现trait的时候传入参数，这里用户可以直接传入方法名注册
#[proc_macro_attribute]
pub fn rpc_service(attr: TokenStream, item: TokenStream) -> TokenStream 
{   
    // 解析标注的结构体
    let input_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input_struct.ident;

    // 解析属性参数，例如 #[rpc_service(login, logout)]
    let args = syn::parse_macro_input!(attr as RpcServiceArgs);
    let methods = args.methods;

    // 将方法名转换为字符串，并生成方法映射
    let register_methods = methods.iter()
        .map(|method_ident| {
                let method_name = method_ident.to_string();

                use quote::spanned::Spanned;
                let method_ident = syn::Ident::new(&format!("{}", method_name),  Span::call_site().into());
                quote! {
                    (
                        #method_name,
                        Box::new(move |request: serde_json::Value| {
                            #method_ident(request)
                        }),
                    )
                }
            }
        );
   
    // 生成 `Service` trait 的实现
    let expanded = quote! {
        #input_struct

        impl Service for #struct_name {
            fn get_service_name(&self) -> &'static str {
                stringify!(#struct_name)
            }

            fn get_methods(&self) -> Vec<(&'static str, RpcMethod)> {
                vec![
                    #(#register_methods),*
                ]
            }
        }
    };

    TokenStream::from(expanded)
}

// 这里定义了一个解析器
struct RpcServiceArgs {
    methods: Vec<String>,
}

impl syn::parse::Parse for RpcServiceArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // 解析为一个标识符列表（标识符会被转换为字符串）
        let methods = input
            .parse_terminated(syn::Ident::parse, syn::Token![,])?
            .into_iter()
            .map(|ident| ident.to_string())
            .collect();

        Ok(RpcServiceArgs { methods })
    }
}
