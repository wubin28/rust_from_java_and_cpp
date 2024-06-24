#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "double_free_risks_cpp" ? 0 : 1;
}
