cmake_minimum_required(VERSION 3.27)

project(AdventofCode CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRE True)
set(CMAKE_EXPORT_COMPILE_COMMANDS True)
add_compile_options(-Wall -Wextra -pedantic -Wshadow -Wconversion -Wcast-align -Weffc++)
add_compile_options(-O3)
include_directories(utils 2023/inputs)

add_executable(trebuchet 2023/01-trebuchet.cpp)
