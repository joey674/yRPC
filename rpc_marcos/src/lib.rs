extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_attribute]
pub fn method(_: TokenStream, item: TokenStream) -> TokenStream 
{
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_args = &input_fn.sig.inputs;
    let fn_block = &input_fn.block;
    let fn_return_type = &input_fn.sig.output;

    let arg_type = if let Some(syn::FnArg::Typed(arg)) = fn_args.first() {
        &arg.ty
    } else {
        panic!("rpc_method must have at least one argument");
    };

    let fn_name_origin = syn::Ident::new(&format!("{}_origin", fn_name), fn_name.span());

    let expanded = quote! {
        pub fn #fn_name_origin(#fn_args) #fn_return_type {
            #fn_block
        }

        pub fn #fn_name(request: serde_json::Value) -> serde_json::Value {
            let request: #arg_type = serde_json::from_value(request).unwrap();
            let response = #fn_name_origin(request);
            serde_json::to_value(response).unwrap()
        }
    };
    TokenStream::from(expanded)
}


#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream 
{   
    let input_struct = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = &input_struct.ident;

    let args = syn::parse_macro_input!(attr as RpcServiceArgs);
    let methods = args.methods;

    let register_methods = methods.iter()
        .map(|method_ident| {
                let method_name = method_ident.clone().to_string();

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


struct RpcServiceArgs {
    methods: Vec<syn::Ident>,
}

impl syn::parse::Parse for RpcServiceArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let methods = input
            .parse_terminated(syn::Ident::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(RpcServiceArgs { methods })
    }
}
