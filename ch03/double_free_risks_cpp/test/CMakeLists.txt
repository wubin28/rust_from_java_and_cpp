# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(double_free_risks_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(double_free_risks_cpp_test source/double_free_risks_cpp_test.cpp)
target_link_libraries(double_free_risks_cpp_test PRIVATE double_free_risks_cpp_lib)
target_compile_features(double_free_risks_cpp_test PRIVATE cxx_std_20)

add_test(NAME double_free_risks_cpp_test COMMAND double_free_risks_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
