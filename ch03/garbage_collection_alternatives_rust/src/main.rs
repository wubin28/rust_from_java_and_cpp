fn main() {
    // 创建一个大数组，使用堆内存
    let mut large_array: Vec<i32> = Vec::with_capacity(1000000000); // large_array作用域开始
    // 将数字0到999999999添加到large_array中
    for i in 0..1000000000 {
        large_array.push(i);
    }

    {
        let _temp = large_array; // 内存所有权发生转移；_temp作用域开始；large_array失效但仍在作用域中
    } // _temp作用域终止并释放内存

    // 创建一个新的大数组，不会因为内存不足崩溃
    let mut another_array: Vec<i32> = Vec::with_capacity(1000000000);
    // 将数字0到999999999添加到another_array中
    for i in 0..1000000000 {
        another_array.push(i);
    }

    println!("Program completed");
} // large_array和another_array的作用域都在此终止，且another_array会释放内存
