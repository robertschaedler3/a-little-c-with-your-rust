cmake_minimum_required(VERSION 3.0)

project(hello)

add_library(${PROJECT_NAME} STATIC hello.c)
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS ${PROJECT_NAME} DESTINATION .)
