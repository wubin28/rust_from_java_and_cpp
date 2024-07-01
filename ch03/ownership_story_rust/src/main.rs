fn ownership_move() {
    let spark1 = String::from("A house on the heap"); // 动态分配内存并绑定所有权给变量 spark1
    let seeker3 = spark1; // 转移所有权给变量 seeker3

    // println!("spark1: {}", spark1); // 编译错误：spark1 在此处已经无效
    println!("seeker3: {}", seeker3); // seeker3 拥有内存的所有权，输出 "A house on the heap"
} // 函数结束，seeker3 离开作用域，内存自动被释放

fn shallow_copy() {
    let xcel2 = 66; // 整型变量 xcel2，值保存在栈上
    let yuma4 = xcel2; // xcel2 的值被拷贝到 yuma4，这里发生的是浅拷贝

    println!("xcel2: {}", xcel2); // xcel2 仍然有效，输出 66
    println!("yuma4: {}", yuma4); // yuma4 拥有拷贝的值，输出 66
} // 函数结束，xcel2 和 yuma4 离开作用域，没有动态内存需要释放

fn main() {
    ownership_move();
    shallow_copy();
}
// 运行结果：
// seeker3: A house on the heap
// xcel2: 66
// yuma4: 66
