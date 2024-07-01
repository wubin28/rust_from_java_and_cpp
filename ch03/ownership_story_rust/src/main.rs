fn ownership_move() {
    let s1 = String::from("A house on the heap"); // 动态分配内存并绑定所有权给变量 s1
    let s2 = s1; // 转移所有权给变量 s2

    // println!("s1: {}", s1); // 编译错误：s1 在此处已经无效
    println!("s2: {}", s2); // s2 拥有内存的所有权，输出 "A house on the heap"
} // 函数结束，s2 离开作用域，内存自动被释放

fn shallow_copy() {
    let x = 66; // 整型变量 x，值保存在栈上
    let y = x; // x 的值被拷贝到 y，这里发生的是浅拷贝

    println!("x: {}", x); // x 仍然有效，输出 66
    println!("y: {}", y); // y 拥有拷贝的值，输出 66
} // 函数结束，x 和 y 离开作用域，没有动态内存需要释放

fn main() {
    ownership_move();
    shallow_copy();
}
// 运行结果：
// s2: A house on the heap
// x: 66
// y: 66