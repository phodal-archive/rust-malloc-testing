cmake_minimum_required(VERSION 3.17)

project(cmalloc-demo)

set(_SRCS
        src/utf_string.h
        src/utf_string.c
        src/main.c)

add_executable(cmalloc-demo ${_SRCS})
