use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();
    // DeriveInput {
    //     // --snip--

    //     ident: Ident {
    //         ident: "Pancakes",
    //         span: #0 bytes(95..103)
    //     },
    //     data: Struct(
    //         DataStruct {
    //             struct_token: Struct,
    //             fields: Unit,
    //             semi_token: Some(
    //                 Semi
    //             )
    //         }
    //     )
    // }
    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
// // 过程宏 里面不能包含其他不相关方法函数
// pub fn add(ast: &syn::DeriveInpute) -> TokenStream {
//     let t = proc_macro::token_stream;
//     return;
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
