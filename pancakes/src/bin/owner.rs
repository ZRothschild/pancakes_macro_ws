fn main() {
    // 字符串字面值，是一个字符串切片  时不可变的
    let s = "hello";
    println!("the s value is: {}", s);

    let mut s = String::from("hello");

    s.push_str(", world");

    println!("the s value is: {}", s);

    //对某个值来说，当拥有它的变量走出作范围，内存会立即自动释放交给操作系统 运行 drop 函数

    // 变量和数据交互方式
    // 1. 移动

    let s1 = String::from("hello");
    // s1 移动给了 s2 s1 废弃
    let s2 = s1;
    println!("the s2 value is: {}", s2);
    // 深拷贝 clone 针对 heap 上面的数据
    let s3: String = s2.clone();
    println!("the s3 value is: {}", s3);

    // stack 上面的数据 复制操作 copy
    let x = 4;
    let y = x;

    println!("the x value is: {}  y value is: {}", x, y);

    let s1 = String::from("hello world!");
    // 把函数引用作为参数 叫做借用
    let len = calculate_length(&s1);

    println!("the length of: {} is: {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
