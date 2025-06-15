cmake_minimum_required(VERSION 3.31)
project(binding_example C)

set(CMAKE_C_STANDARD 99)

add_executable(example_server_listening tcp_server.c)
add_executable(example_connect_server tcp_client.c)

include_directories(${PROJECT_SOURCE_DIR}/include)
link_directories(${PROJECT_SOURCE_DIR}/libs)

target_link_libraries(example_server_listening
        ${PROJECT_SOURCE_DIR}/libs/nogamepads_c.dll.lib
)

target_link_libraries(example_connect_server
        ${PROJECT_SOURCE_DIR}/libs/nogamepads_c.dll.lib
)

if(WIN32)
    set(DLL_FILE "${PROJECT_SOURCE_DIR}/libs/nogamepads_c.dll")
    add_custom_command(TARGET example_server_listening POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy
            "${DLL_FILE}"
            $<TARGET_FILE_DIR:example_server_listening>
            COMMENT "Copying DLL to output directory"
    )
    add_custom_command(TARGET example_connect_server POST_BUILD
            COMMAND ${CMAKE_COMMAND} -E copy
            "${DLL_FILE}"
            $<TARGET_FILE_DIR:example_connect_server>
            COMMENT "Copying DLL to output directory"
    )
endif()