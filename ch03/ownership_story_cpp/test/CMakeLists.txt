# Parent project does not export its library target, so this CML implicitly
# depends on being added from it, i.e. the testing is done only from the build
# tree and is not feasible from an install location

project(ownership_story_cppTests LANGUAGES CXX)

# ---- Tests ----

add_executable(ownership_story_cpp_test source/ownership_story_cpp_test.cpp)
target_link_libraries(ownership_story_cpp_test PRIVATE ownership_story_cpp_lib)
target_compile_features(ownership_story_cpp_test PRIVATE cxx_std_20)

add_test(NAME ownership_story_cpp_test COMMAND ownership_story_cpp_test)

# ---- End-of-file commands ----

add_folders(Test)
