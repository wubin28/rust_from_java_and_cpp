fn ownership_move() {
    let spark1 = String::from("A house on the heap"); // 动态分配内存并绑定所有权给变量 spark1
    let s2 = spark1; // 转移所有权给变量 s2

    // println!("spark1: {}", spark1); // 编译错误：spark1 在此处已经无效
    println!("s2: {}", s2); // s2 拥有内存的所有权，输出 "A house on the heap"
} // 函数结束，s2 离开作用域，内存自动被释放

fn shallow_copy() {
    let xcel2 = 66; // 整型变量 xcel2，值保存在栈上
    let y = xcel2; // xcel2 的值被拷贝到 y，这里发生的是浅拷贝

    println!("xcel2: {}", xcel2); // xcel2 仍然有效，输出 66
    println!("y: {}", y); // y 拥有拷贝的值，输出 66
} // 函数结束，xcel2 和 y 离开作用域，没有动态内存需要释放

fn main() {
    ownership_move();
    shallow_copy();
}
// 运行结果：
// s2: A house on the heap
// xcel2: 66
// y: 66
