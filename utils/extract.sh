#!/bin/sh

{
command -v cc   || exit 2
command -v grep || exit 3
command -v cut  || exit 4
command -v sed  || exit 5
} >/dev/null 2>&1

echo "#include <syscall.h>" | cc -E -dM - | grep -e "#define __NR" | grep -e "^#define __NR.* [0-9]*$" | sed -e 's/#define __NR_//' -e 's/#define __NR3264_//'
