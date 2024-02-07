// 这里引入的是 trait
use hello_macro::HelloMacro;
// 引入过程宏 自定义派生宏
use hello_macro_derive::HelloMacro;
// 引入作用域，log_func_info 才可以用，作用域区服 包 crate mod
use hello_macro_attribute::log_func_info;

// 让结构体使用过程宏 自定义派生宏
#[derive(HelloMacro)]
pub struct Pancakes;

pub mod http {

    // 宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用
    use hello_macro_attribute::route;

    // #[route(GET, "/")] // atrr 代表 GET "/"
    // fn index() {} // item 代表 fn index(){}
    #[route(GET, "/")]
    pub fn index() {
        println!("route Hello, world! index");
    }
}

// 宏和函数的最后一个重要的区别是：在一个文件里调用宏 之前 必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用
// use hello_macro_attribute::route;

// // #[route(GET, "/")] // atrr 代表 GET "/"
// // fn index() {} // item 代表 fn index(){}
// #[route(GET, "/")]
// pub fn index() {
//     println!("route Hello, world! index");
// }

#[log_func_info(GET, "/")]
pub fn index_log() {
    println!("log_func_info Hello, world! index");
}
