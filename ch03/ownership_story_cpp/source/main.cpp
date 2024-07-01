#include <iostream>  // 包含输入输出流库
#include <string>  // 包含字符串库

// 展示双重释放问题的函数
void demonstrate_double_free_issue()
{
  // 在堆上分配一个字符串对象
  std::string* spark1 = new std::string("A house on the heap");

  // 将 spark1 的地址赋给 s2，这意味着 spark1 和 s2 指向同一个内存位置
  std::string* s2 = spark1;

  // 输出 spark1 指向的字符串内容
  std::cout << "spark1: " << *spark1 << std::endl;

  // 输出 s2 指向的字符串内容
  std::cout << "s2: " << *s2 << std::endl;

  // 释放 s2 指向的内存（即堆上的字符串对象）
  delete s2;

  // 为避免再次使用 s2，建议将其设置为 nullptr
  // s2 = nullptr;

  // 为避免再次使用 spark1，建议将其设置为 nullptr
  // spark1 = nullptr;

  // 释放 spark1 指向的内存（这是双重释放，因为 s2 已经释放了该内存）
  delete spark1;  // 这将导致双重释放（double free）
}

// 展示浅拷贝的函数
void shallow_copy()
{
  int xcel2 = 66;  // 定义一个整型变量 xcel2，值保存在栈上
  int y = xcel2;  // 将 xcel2 的值拷贝到 y，这里发生的是浅拷贝

  // 输出 xcel2 的值
  std::cout << "xcel2: " << xcel2 << std::endl;  // xcel2 依然有效，输出 66
  // 输出 y 的值
  std::cout << "y: " << y << std::endl;  // y 拥有拷贝的值，输出 66
}  // 函数结束，xcel2 和 y 离开作用域，没有动态内存需要释放

int main()
{
  demonstrate_double_free_issue();  // 调用展示双重释放问题的函数
  shallow_copy();  // 调用展示浅拷贝的函数
  return 0;  // 程序正常结束
}
// 运行结果（双重释放问题编译器没有发现，只有在运行时才发现）：
// spark1: A house on the heap
// s2: A house on the heap
// ownership_story_cpp(48352,0x1f027cc00) malloc: *** error for object 0x600001640040: pointer being freed was not allocated
// ownership_story_cpp(48352,0x1f027cc00) malloc: *** set a breakpoint in malloc_error_break to debug
// [1]    48352 abort      ./ownership_story_cpp
