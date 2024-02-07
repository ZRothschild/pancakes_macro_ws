// 将 proc_macro 导入 项目 package
extern crate proc_macro;
// 使用 crate::proc_macro 到处条目
use crate::proc_macro::TokenStream;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    eprintln!("===input {:#?}", input);
    input
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
