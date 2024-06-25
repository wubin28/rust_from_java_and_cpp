fn main() {
    // 创建一个大数组，使用堆内存
    let mut large_array: Vec<i32> = Vec::with_capacity(1000000000);
    for i in 0..1000000000 {
        large_array.push(i);
    }

    // 所有权转移，large_array离开作用域，自动释放内存
    {
        // 内存转移到临时变量
        let _temp = large_array;
        // _temp在这个作用域结束时自动释放内存
    }

    // 创建一个新的大数组，不会因为内存不足崩溃
    let mut another_array: Vec<i32> = Vec::with_capacity(1000000000);
    for i in 0..1000000000 {
        another_array.push(i);
    }

    println!("Program completed");
}
