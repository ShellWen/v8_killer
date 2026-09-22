find_package(Git REQUIRED)
set(patch "${CMAKE_CURRENT_LIST_DIR}/dobby.patch")
set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${patch}")
execute_process(
  COMMAND "${GIT_EXECUTABLE}" apply --reverse --check "${patch}"
  WORKING_DIRECTORY "${dobby_SOURCE_DIR}"
  RESULT_VARIABLE applied
  OUTPUT_QUIET ERROR_QUIET)
if(NOT applied EQUAL 0)
  execute_process(
    COMMAND "${GIT_EXECUTABLE}" apply "${patch}"
    WORKING_DIRECTORY "${dobby_SOURCE_DIR}"
    RESULT_VARIABLE result
    ERROR_VARIABLE error)
  if(NOT result EQUAL 0)
    message(FATAL_ERROR "Cannot apply Dobby patch: ${error}")
  endif()
endif()
