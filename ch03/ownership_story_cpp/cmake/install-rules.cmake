install(
    TARGETS ownership_story_cpp_exe
    RUNTIME COMPONENT ownership_story_cpp_Runtime
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
