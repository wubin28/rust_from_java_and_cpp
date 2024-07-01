#include "lib.hpp"

auto main() -> int
{
  auto const lib = library {};

  return lib.name == "ownership_story_cpp" ? 0 : 1;
}
