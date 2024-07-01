#include <iostream>
#include <string>

void demonstrate_double_free_issue()
{
  std::string* s1 = new std::string("A house on the heap");

  std::string* s2 = s1;

  std::cout << "s1: " << *s1 << std::endl;

  std::cout << "s2: " << *s2 << std::endl;

  delete s2;
  // s2 = nullptr; // 避免 s2 再次使用
  // s1 = nullptr; // 避免 s1 再次使用
  delete s1;  // 这将导致双重释放（double free）
}

void shallow_copy()
{
  int x = 66;  // 整型变量 x，值保存在栈上
  int y = x;  // x 的值被拷贝到 y，这里发生的是浅拷贝

  std::cout << "x: " << x << std::endl;  // x 依然有效，输出 42
  std::cout << "y: " << y << std::endl;  // y 拥有拷贝的值，输出 42
}  // 函数结束，x 和 y 离开作用域，没有动态内存需要释放

int main()
{
  demonstrate_double_free_issue();
  shallow_copy();
  return 0;
}
// 运行结果（双重释放问题编译器没有发现，只有在运行时才发现）：
// s1: A house on the heap
// s2: A house on the heap
// ownership_story_cpp(14459,0x1f027cc00) malloc: *** error for object 0x600000bfd200: pointer being freed was not allocated
// ownership_story_cpp(14459,0x1f027cc00) malloc: *** set a breakpoint in malloc_error_break to debug
// [1]    14459 abort      ./ownership_story_cpp