use core::{
    arch::asm,
    ffi::{c_int, c_void},
};

use crate::{
    ffi::pid_t,
    io::{write, STDERR_FILENO},
    signal::abort,
    sys::{SYS_exit, SYS_getpid},
};

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn exit(status: c_int) -> ! {
    // TODO: on_exit
    _exit(status);
}

#[no_mangle]
pub unsafe extern "C" fn _exit(status: c_int) -> ! {
    asm! {
        "syscall",
        "ud2", // crash if syscall returns
        in("rax") SYS_exit,
        in ("rdi") status,
        options(noreturn, preserves_flags),
    }
}

#[no_mangle]
pub unsafe extern "C" fn _Exit(status: c_int) -> ! {
    _exit(status);
}

#[no_mangle]
pub unsafe extern "C" fn getpid() -> pid_t {
    let pid: pid_t;
    asm!(
        "syscall",
        in("rax") SYS_getpid,
        lateout("rax") pid,
        options(nostack, preserves_flags),
    );
    pid
}

// #[no_mangle]
// unsafe extern "C" fn atexit(f: extern "C" fn()) -> c_int {
//     0
// }

#[no_mangle]
unsafe extern "C" fn __cxa_finalize(_d: *mut c_void) {}

#[no_mangle]
unsafe extern "C" fn __cxa_atexit(
    _func: unsafe extern "C" fn(*mut c_void),
    _arg: *mut c_void,
    _dso: *mut c_void,
) -> c_int {
    0
}

#[cold]
#[no_mangle]
unsafe extern "C" fn __stack_chk_fail() -> ! {
    let message = b"__stack_chk_fail";
    write(STDERR_FILENO, message.as_ptr().cast(), message.len());
    abort();
}

#[cold]
#[no_mangle]
unsafe extern "C" fn __stack_chk_fail_local() -> ! {
    let message = b"__stack_chk_fail_local";
    write(STDERR_FILENO, message.as_ptr().cast(), message.len());
    abort();
}
