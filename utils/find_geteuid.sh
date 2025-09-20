#!/bin/sh

{
command -v cc      || exit 2
command -v objdump || exit 3
command -v grep    || exit 4
} >/dev/null 2>&1

LIBC=$(cc --print-file-name=libc.a)
objdump --disassemble=__geteuid --no-show-raw-insn --no-addresses "$LIBC" | grep -e '__geteuid' -A 4
