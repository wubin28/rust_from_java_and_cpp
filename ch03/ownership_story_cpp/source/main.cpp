#include <iostream>
#include <string>

void demonstrate_double_free_issue()
{
  std::string* spark1 = new std::string("A house on the heap");

  std::string* seeker3 = spark1;

  std::cout << "spark1: " << *spark1 << std::endl;

  std::cout << "seeker3: " << *seeker3 << std::endl;

  delete seeker3;
  // seeker3 = nullptr; // 避免 seeker3 再次使用
  // spark1 = nullptr; // 避免 spark1 再次使用
  delete spark1;  // 这将导致双重释放（double free）
}

void shallow_copy()
{
  int xcel2 = 66;  // 整型变量 xcel2，值保存在栈上
  int yuma4 = xcel2;  // xcel2 的值被拷贝到 yuma4，这里发生的是浅拷贝

  std::cout << "xcel2: " << xcel2 << std::endl;  // xcel2 依然有效，输出 42
  std::cout << "yuma4: " << yuma4 << std::endl;  // yuma4 拥有拷贝的值，输出 42
}  // 函数结束，xcel2 和 yuma4 离开作用域，没有动态内存需要释放

int main()
{
  demonstrate_double_free_issue();
  shallow_copy();
  return 0;
}
// 运行结果（双重释放问题编译器没有发现，只有在运行时才发现）：
// spark1: A house on the heap
// seeker3: A house on the heap
// ownership_story_cpp(14459,0x1f027cc00) malloc: *** error for object 0x600000bfd200: pointer being freed was not allocated
// ownership_story_cpp(14459,0x1f027cc00) malloc: *** set a breakpoint in malloc_error_break to debug
// [1]    14459 abort      ./ownership_story_cpp