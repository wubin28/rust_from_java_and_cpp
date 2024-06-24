#include <iostream>

void doubleFreeExample()
{
  int* ptr = new int(42);  // 动态分配内存
  std::cout << "Value: " << *ptr << std::endl;

  delete ptr;  // 第一次释放内存
  std::cout << "First delete done." << std::endl;

  // 尝试访问已删除的指针，可能会导致未定义行为
  try {
    std::cout << "Trying to access deleted memory: " << *ptr << std::endl;
  } catch (...) {
    std::cout << "Accessing deleted memory caused an exception." << std::endl;
  }

  delete ptr;  // 第二次释放同一块内存，导致双重释放
  std::cout << "Second delete done." << std::endl;
}

int main()
{
  doubleFreeExample();
  return 0;
}
