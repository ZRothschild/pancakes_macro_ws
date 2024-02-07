// 将 proc_macro 导入 项目 package
extern crate proc_macro;
// 使用 crate::proc_macro 到处条目
use crate::proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 调用宏时打印输出 item attr   eprintln 这个命令在 编译前
    eprintln!("route item: \"{:#?}\" \n  attr: \"{:#?}\"", item, attr);
    // 调用宏时输出原来输出的
    item
}

#[proc_macro_attribute]
pub fn log_func_info(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_block = &func.block;
    let output = quote! { // 这个是在执行时
        {
            println!("log fun {} starts", stringify!(#func_name));
            let __log_result = { #func_block };
            println!("log fun {} ends", stringify!(#func_name));
            __log_result
        }
    };
    func.block = syn::parse2(output).unwrap();
    quote! { #func }.into()
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
