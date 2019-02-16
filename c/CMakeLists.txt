cmake_minimum_required(VERSION 3.13)
project(ComputerScience)

set(ComputerScience_VERSION_MAJOR 1)
set(ComputerScience_VERSION_MINOR 0)
set(CMAKE_C_STANDARD 99)

add_executable(ComputerScience main.c trees/red-black-tree.h)
add_library(Trees trees/red-black-tree.c trees/red-black-tree.h trees/binary-tree.c trees/binary-tree.h)
add_library(Other other/queue.c other/queue.h)
add_library(Sorting sorting/helpers.c sorting/helpers.h sorting/bubble.c sorting/shaker.c sorting/insertion.c
        sorting/gnome.c sorting/merge.c sorting/btree.c sorting/timsort.c sorting/sorting.h)
target_link_libraries(ComputerScience Sorting Trees Other)