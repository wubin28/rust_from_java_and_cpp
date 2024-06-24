cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    double_free_risks_cpp
    VERSION 0.1.0
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    double_free_risks_cpp_lib OBJECT
    source/lib.cpp
)

target_include_directories(
    double_free_risks_cpp_lib ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/source>"
)

target_compile_features(double_free_risks_cpp_lib PUBLIC cxx_std_20)

# ---- Declare executable ----

add_executable(double_free_risks_cpp_exe source/main.cpp)
add_executable(double_free_risks_cpp::exe ALIAS double_free_risks_cpp_exe)

set_property(TARGET double_free_risks_cpp_exe PROPERTY OUTPUT_NAME double_free_risks_cpp)

target_compile_features(double_free_risks_cpp_exe PRIVATE cxx_std_20)

target_link_libraries(double_free_risks_cpp_exe PRIVATE double_free_risks_cpp_lib)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT double_free_risks_cpp_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of double_free_risks_cpp"
  )
endif()

include(cmake/dev-mode.cmake)
