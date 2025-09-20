# syscallnrs

Uses your system's `syscall.h` to build mappings from system call names to numbers. It provides three such mappings:

  - `SYSCALLS`, a static slice of pairs of system call names and numbers;
  - `syscall_of_nr`, a function which looks up a system call name given a number; and
  - `nr_of_syscall`, a function which looks up a number given a system call.

The code for these is generated statically. Both functions are generated as large matches rather than data structure lookups.

To use this in your project:

```
[dependencies]
syscallnrs = "0.1"
```

## Requirements

The build-time code generation requires a C compiler (GCC or clang should work) and standard utilities (`grep`, `cut`, and `sed`). The tests require `objdump` (to confirm that the extracted system calls are correct).