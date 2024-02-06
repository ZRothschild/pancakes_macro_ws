use vec_macro_rules::{vecc,vec};
use pancakes::tool::help::Pancakes;
use hello_macro::HelloMacro;


fn main() {

    Pancakes::hello_macro();

    // 声明宏事例
    let v4 = vec![1,2,3,4];
    println!("vec_macro_rules::vec {:?}",v4);

    // 模块方法 需要声明是否是 pub
    let sum = vecc::add(1, 2);
    println!("vec_macro_rules::vecc::add is {}",sum);
}
