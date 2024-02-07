use vec_macro_rules::{vec, vecc};
// 引入结构体
use pancakes::tool::help::{self, http, Pancakes};
// 引入trait
use hello_macro::HelloMacro;

use hello_macro_fn::sql;

fn main() {
    // hello_macro_fn::sql 过程宏函数
    sql!("test data");

    // pancakes::tool::help 下面得 index_log,使用 self
    help::index_log();

    // pancakes::tool::help::http mod 下面得index
    http::index();

    // 过程宏 自定义派生宏
    Pancakes::hello_macro();

    // 声明宏事例
    let v4 = vec![1, 2, 3, 4];
    println!("vec_macro_rules::vec {:?}", v4);

    // 模块方法 需要声明是否是 pub
    let sum = vecc::add(1, 2);
    println!("vec_macro_rules::vecc::add is {}", sum);
}
