use atomic_dbg::dbg;
use core::{
    arch::naked_asm,
    ffi::{c_char, c_int},
    panic::PanicInfo,
    ptr,
};

use crate::process::{exit, EXIT_FAILURE};

#[no_mangle]
pub static mut environ: *mut *const c_char = ptr::null_mut();

extern "C" {
    static __init_array_start: *const extern "C" fn();
    static __init_array_end: *const extern "C" fn();

    fn main(argc: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
}

#[lang = "eh_personality"]
fn rust_eh_personality() {}

#[no_mangle]
#[naked]
#[cfg(target_arch = "x86_64")]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm! {
        "mov rdi, rsp",
        "push rbp",
        "jmp {entry}",
        entry = sym entry,
    }
}

#[no_mangle]
pub unsafe extern "C" fn entry(stack: *mut usize) -> ! {
    let argc = *stack;

    let argv = stack.offset(1);

    // // Skip over the argv array in memory.
    // // Offset from stack_ptr: 1 for argc, then argc for argv pointers.
    let envp_offset = 2 + (argc as isize);
    let envp = stack.offset(envp_offset) as *mut *mut c_char;
    // let envp = stack.offset(2);

    environ = envp as _;

    let exit_code = main(argc as _, argv as _, envp as _);

    exit(exit_code);
}

// #[no_mangle]
// pub unsafe extern "C" fn __libc_init_array() {
//     let mut current = __init_array_start;
//     while current < __init_array_end {
//         let init_fn: extern "C" fn() = *current;
//         init_fn();
//         current = current.add(1);
//     }
// }

// # experiments...

// #[no_mangle]
// pub unsafe extern "C" fn __libc_start_main(
//     main_fn: unsafe extern "C" fn(argc: c_int, argv: *mut *mut c_char) -> c_int,
//     argc: c_int,
//     argv: *const *const c_char,
//     envp: *const *const c_char,
//     // init_fn: Option<extern "C" fn()>,
//     // fini_fn: Option<extern "C" fn()>,
//     // rtld_fini_fn: Option<extern "C" fn()>,
//     // stack_end: *const (),
// ) -> c_int {
//     // environ = envp as *mut *const c_char;

//     // // let exit_code = main_fn(argc, argv as *mut *mut c_char);
//     // let exit_code = main(argc, argv);
//     // exit(exit_code);

//     // exit(42);

//     return 0;
// }

// #[cfg(target_arch = "x86_64")]
// global_asm! {
//     r#"
//         .section .text
//         .global _start
//         .type _start, @function
//         _start:
//             mov rsi, [rsp]
//             lea rdx, [rsp + 8]
//             lea rcx, [rdx + 8 * rsi]
//             mov rdi, main
//             push rbp
//             mov rbp, rsp
//             call __libc_start_main
//             pop rbp
//             hlt
//     "#
// }

// #[cfg(target_arch = "x86_64")]
// global_asm! {
//     r#"
//         .section .text
//         .global _start
//         .type _start, @function
//         _start:
//             mov rdi, rsp
//             push rbp
//             jmp {entry}
//         .size _start, .-_start
//     "#,
//     entry = sym entry,
// }
