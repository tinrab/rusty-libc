use core::arch::asm;

use crate::{
    process::{exit, getpid, EXIT_FAILURE},
    sys::{SYS_kill, SIGABRT},
};

#[no_mangle]
pub unsafe extern "C" fn abort() -> ! {
    let pid = getpid();
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_kill,
            in("rdi") pid,
            in("rsi") SIGABRT,
            options(nostack, preserves_flags)
        );
    }
    unsafe { exit(EXIT_FAILURE) }
}
