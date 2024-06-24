install(
    TARGETS double_free_risks_cpp_exe
    RUNTIME COMPONENT double_free_risks_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
