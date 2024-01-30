cmake_minimum_required(VERSION 3.26)

project(Helpers CXX)

set(CXX_STANDARD_REQUIRED TRUE)

find_package(Python3 COMPONENTS Development.SABIModule REQUIRED)

add_library(day6_part2_helper SHARED src/day6_part2_helper.cpp)
set_property(TARGET day6_part2_helper PROPERTY CXX_STANDARD 23)

add_library(day6_part2_helper_alt SHARED src/day6_part2_helper_alt.cpp)
set_property(TARGET day6_part2_helper_alt PROPERTY CXX_STANDARD 20)
set_target_properties(day6_part2_helper_alt PROPERTIES  PREFIX "" OUTPUT_NAME "helper")
target_link_libraries(day6_part2_helper_alt Python3::SABIModule)