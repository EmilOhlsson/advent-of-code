cmake_minimum_required(VERSION 3.27)

# TODO: Create some form of profiles, for example address sanitizing
project(AdventofCode CXX)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRE True)
set(CMAKE_EXPORT_COMPILE_COMMANDS True)
add_compile_options(-Wall -Wextra -pedantic -Wshadow -Wconversion -Wcast-align -Weffc++ -g)
add_compile_options(-Og)
include_directories(utils 2023/inputs)
add_compile_options(-fsanitize=address)
add_link_options(-fsanitize=address)
find_package(fmt)

add_executable(trebuchet 2023/01-trebuchet.cpp)
add_executable(cube-conundrum 2023/02-cube-conundrum.cpp)

target_link_libraries(cube-conundrum fmt::fmt)
