fn main() {
    println!("bin hello");

    let mut x = 5;
    println!("The value is {}", x);
    x = 6;
    println!("The value is {}", x);

    // ================================================ tup start  ===================================
    // let t = (500, 6.4, 1); tup 长度不可变
    let t: (i32, f64, u8) = (500, 6.4, 1);

    println!("t.0: {} t.1: {} t.2: {}", t.0, t.1, t.2);

    let (x, y, z) = t;
    println!("x: {} y: {} z: {}", x, y, z);

    // ================================================ tup end  ===================================

    // ================================================ array start  ===================================

    let a: [i32; 5] = [3; 5]; // eg. 5个都为3元素 let a = [3,3,3,3,3]
    let arr = ["hello", "world"];
    println!("a: {:?} arr: {:?}", a, arr[0]); // 数组越界 编译会报错 但会编译时简单检查，运行报错

    for (index, value) in arr.iter().enumerate() {
        println!("arr index is: {} value is: {}", index, value);
    }

    for data in a.iter() {
        println!("a data is: {}", data);
    }

    // rev 取反  Reverses an iterator's direction.
    for data in arr.iter().rev() {
        println!("arr data is: {}", data);
    }

    // (1..4) Range  rev 取反  Reverses an iterator's direction.
    for range in (1..4).rev() {
        println!("range data is: {}", range);
    }

    // ================================================ array end    ====================================

    // ================================================ fn start  ===================================

    let add = another_fn(5, 6);
    println!("the value of add is: {}", add);

    // ================================================ fn end  ===================================

    // ================================================ fn end  ===================================

    // ================================================ fn end  ===================================
}

fn another_fn(x: i32, y: i32) -> i32 {
    println!("another_fn the value of x is: {} y is: {}", x, y);
    return x + y;
}
