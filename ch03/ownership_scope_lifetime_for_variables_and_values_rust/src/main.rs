fn main() {
    // 栈上值被变量所有
    let xcel2 = 5; // xcel2 是变量，其拥有一个存储在栈上的整型值 5
    println!("xcel2: {}", xcel2); // xcel2 在此作用域内有效

    // 栈上值不涉及所有权移动，只涉及浅拷贝
    let yuma4 = xcel2; // yuma4 是新变量，它获得了 xcel2 的值的副本（浅拷贝）
    println!("yuma4: {}", yuma4); // xcel2 和 yuma4 都有效，但它们是独立的副本

    // 变量的所有权和作用域
    {
        let z = 10; // z 是变量，其拥有一个存储在栈上的整型值 10
        println!("z: {}", z); // z 在此作用域内有效
    } // z的作用域结束，栈上值 10 被从栈顶弹出并销毁

    // 堆上值被变量所有
    let spark1 = String::from("hello"); // spark1 是变量，其拥有一个存储在堆上的字符串值 "hello"
    println!("spark1: {}", spark1); // spark1 在此作用域内有效

    // 堆上值的所有权移动
    let seeker3 = spark1; // seeker3 是新变量，spark1 的所有权被移动给 seeker3
    // println!("spark1: {}", spark1); // 错误！spark1 的所有权已经被移动，不能再访问

    println!("seeker3: {}", seeker3); // seeker3 在此作用域内有效

    // 堆上值参与所有权移动
    let s3 = seeker3; // s3 是新变量，seeker3 的所有权被移动给 s3
    // println!("seeker3: {}", seeker3); // 错误！seeker3 的所有权已经被移动，不能再访问

    println!("s3: {}", s3); // s3 在此作用域内有效

    // 变量的生存期
    let t; // t 声明但未初始化
    {
        let s4 = String::from("world"); // s4 是变量，其拥有一个存储在堆上的字符串值 "world"
        t = s4; // s4 的所有权被移动给 t
    } // s4 的作用域结束，堆上值 "world" 因所有权转移给t而不会被自动释放内存
    // println!("s4: {}", s4); // 错误！s4 的作用域已结束，不能再访问
    println!("t: {}", t); // t 在此作用域内有效

    // 栈上值的生存期
    let a = 20; // a 是变量，其拥有一个存储在栈上的整型值 20
    println!("a: {}", a); // a 在此作用域内有效

    // 堆上值的生存期
    let b = Box::new(30); // b 是变量，其拥有一个存储在堆上的整型值 30
    println!("b: {}", b); // b 在此作用域内有效
} // a 的作用域和生存期结束，其栈上值被从栈顶弹出并销毁；b 的作用域和生存期结束，其堆上值被自动释放内存
// 运行结果：
// xcel2: 5
// yuma4: 5
// z: 10
// spark1: hello
// seeker3: hello
// s3: hello
// t: world
// a: 20
// b: 30