cmake_minimum_required(VERSION 3.31)
project(binding_example C)

set(CMAKE_C_STANDARD 99)

add_executable(binding_example main.c)

include_directories(${PROJECT_SOURCE_DIR}/include)
link_directories(${PROJECT_SOURCE_DIR}/libs)

target_link_libraries(binding_example
        ${PROJECT_SOURCE_DIR}/libs/nogamepads_c.dll.lib
)

if(WIN32)
    set(DLL_FILE "${PROJECT_SOURCE_DIR}/libs/nogamepads_c.dll")
    add_custom_command(TARGET binding_example POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy
            "${DLL_FILE}"
            $<TARGET_FILE_DIR:binding_example>
            COMMENT "Copying DLL to output directory"
    )
endif()