# Rusty libc

libc implemented in Rust for fun.

🚧🔨 Forever work in progress.

## Status

APIs:
- [memory.rs](src/memory.rs): getpagesize, mmap, munmap, mprotect, malloc, free, calloc, realloc, reallocarray, memcpy, memcmp, bcmp, memset
- [string.rs](src/string.rs): strlen, strcmp, strncmp, toupper, tolower
- [signal.rs](src/signal.rs): abort
- [process.rs](src/process.rs): exit, getpid
- [io.rs](src/io.rs): write, printf, fputc, fputs, puts, putc, putchar
