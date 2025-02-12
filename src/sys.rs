use core::ffi::{c_int, c_long};

// sys/syscall.h
pub const SYS_getpid: c_long = 39;
pub const SYS_exit: c_long = 60;
pub const SYS_kill: c_long = 62;

// Process abort signal.
pub const SIGABRT: c_int = 6;
