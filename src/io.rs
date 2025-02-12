use core::arch::asm;
use core::ffi::{c_char, c_int, c_size_t, c_ssize_t, c_void, CStr};

use crate::utility::c_str_to_str;

#[repr(C)]
pub struct FILE {
    fd: c_int,
}

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

#[no_mangle]
pub static mut stdin: *mut FILE = &mut FILE { fd: STDIN_FILENO };
#[no_mangle]
pub static mut stdout: *mut FILE = &mut FILE { fd: STDOUT_FILENO };
#[no_mangle]
pub static mut stderr: *mut FILE = &mut FILE { fd: STDERR_FILENO };

pub const EOF: i32 = -1;

const SYSCALL_WRITE: usize = 1;

#[no_mangle]
pub unsafe extern "C" fn write(fd: c_int, buf: *const c_void, count: c_size_t) -> c_ssize_t {
    let result: isize;
    asm!(
        "syscall",
        in("rax") SYSCALL_WRITE,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rax") result,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
    );
    result
}

#[no_mangle]
pub unsafe extern "C" fn fputc(c: c_int, stream: *mut FILE) -> c_int {
    if stream.is_null() {
        return EOF;
    }
    let fd = (*stream).fd;
    let buf = [c as u8];
    if write(fd, buf.as_ptr() as *const c_void, 1) == 1 {
        c
    } else {
        EOF
    }
}

#[no_mangle]
pub unsafe extern "C" fn fputs(s: *const c_char, stream: *mut FILE) -> c_int {
    if s.is_null() || stream.is_null() {
        return EOF;
    }
    let mut len: usize = 0;
    let mut p = s;
    while *p != 0 {
        len += 1;
        p = p.add(1);
    }
    let fd = (*stream).fd;
    if write(fd, s as *const c_void, len) as usize == len {
        0
    } else {
        EOF
    }
}

#[no_mangle]
pub unsafe extern "C" fn puts(s: *const c_char) -> c_int {
    let res = fputs(s, stdout);
    if res != EOF {
        if fputc('\n' as _, stdout) == '\n' as _ {
            res
        } else {
            EOF
        }
    } else {
        EOF
    }
}

#[no_mangle]
pub unsafe extern "C" fn putc(c: c_int, stream: *mut FILE) -> c_int {
    fputc(c, stream)
}

#[no_mangle]
pub unsafe extern "C" fn putchar(c: c_int) -> c_int {
    fputc(c, stdout)
}

const PRINTF_BUFFER_SIZE: usize = 128;

#[no_mangle]
pub unsafe extern "C" fn printf(format: *const c_char, mut args: ...) -> c_int {
    let format_str = c_str_to_str(format);

    let mut buf = [0u8; PRINTF_BUFFER_SIZE];
    let mut buf_index = 0;
    let mut format_chars = format_str.chars();

    while let Some(format_char) = format_chars.next() {
        if format_char == '%' {
            if let Some(specifier) = format_chars.next() {
                match specifier {
                    's' => {
                        let arg = args.arg::<*const c_char>();
                        // printf expects null-terminated strings
                        let s_str = c_str_to_str(arg);
                        for char_s in s_str.chars() {
                            if buf_index < PRINTF_BUFFER_SIZE {
                                buf[buf_index] = char_s as u8;
                                buf_index += 1;
                            } else {
                                // If buffer full
                                break;
                            }
                        }
                    }
                    'd' | 'i' => {
                        // TODO is c_int ok?
                        let arg = args.arg::<c_int>();

                        let mut str_buf = [0u8; 20];
                        let mut str_buf_index = 0;
                        let mut n = arg;
                        if n == 0 {
                            str_buf[str_buf_index] = b'0';
                            str_buf_index += 1;
                        } else {
                            while n != 0 {
                                let digit = (n % 10) as u8;
                                str_buf[str_buf_index] = digit + b'0';
                                str_buf_index += 1;
                                n /= 10;
                            }
                        }

                        for b in str_buf[0..str_buf_index].iter().rev() {
                            if buf_index < PRINTF_BUFFER_SIZE {
                                buf[buf_index] = *b;
                                buf_index += 1;
                            } else {
                                // If buffer full
                                break;
                            }
                        }
                    }
                    '%' => {
                        // %%: Literal %
                        if buf_index < PRINTF_BUFFER_SIZE {
                            buf[buf_index] = b'%';
                            buf_index += 1;
                        }
                    }
                    _ => {
                        // Unknown specifier - for now, just print '%' and the specifier itself
                        if buf_index < PRINTF_BUFFER_SIZE - 1 {
                            buf[buf_index] = b'%';
                            buf[buf_index + 1] = specifier as u8;
                            buf_index += 2;
                        } else if buf_index < PRINTF_BUFFER_SIZE {
                            buf[buf_index] = b'%';
                            buf_index += 1;
                        }
                    }
                }
            } else {
                // Just print '%' if it's the last char or followed by nothing - technically incorrect printf behavior
                if buf_index < PRINTF_BUFFER_SIZE {
                    buf[buf_index] = b'%';
                    buf_index += 1;
                }
            }
        } else {
            // Regular character
            if buf_index < PRINTF_BUFFER_SIZE {
                buf[buf_index] = format_char as u8;
                buf_index += 1;
            }
        }
    }

    // Null-terminate the buffer
    if buf_index < PRINTF_BUFFER_SIZE {
        buf[buf_index] = 0;
    }

    match write_buf(STDOUT_FILENO, &buf[0..buf_index]) {
        Ok(written) => written as _,
        Err(_) => EOF,
    }
}

const BUFFER_SIZE: usize = 4096;

pub fn write_buf(fd: c_int, buf: &[u8]) -> Result<usize, usize> {
    // If buf is not too long, use a stack buffer
    // if s.len() < BUFFER_SIZE - 1 {
    //     let mut buf = [0u8; BUFFER_SIZE];
    //     buf[..s.len()].copy_from_slice(s.as_bytes());
    //     buf[s.len()] = 0;

    //     // write until the buffer is empty
    //     let mut i = 0;
    //     let mut n = 0;
    //     while i < BUFFER_SIZE {
    //         n = unsafe { write(STDOUT_FILENO, buf[i] as *const c_void, 1) };
    //         if n < 0 {
    //             return Err(fmt::Error);
    //         }
    //         if n == 0 {
    //             break;
    //         }
    //         i += n as usize;
    //     }
    // } else {
    //     // For very long strings, fall back to writing one character at a time
    //     for &b in s.as_bytes() {
    //         if unsafe { fputc(b as _, stdout) } == EOF {
    //             return Err(fmt::Error);
    //         }
    //     }
    // }

    let mut written = 0;
    let len = buf.len();

    while written < len {
        let remaining = len - written;
        let to_write = remaining.min(BUFFER_SIZE);

        let result = unsafe { write(fd, buf[written..].as_ptr() as *const c_void, to_write) };
        if result < 0 {
            return Err(written);
        }
        if result == 0 {
            break;
        }

        written += result as usize;
    }

    Ok(written)
}

pub fn write_str(fd: c_int, s: &CStr) -> Result<usize, usize> {
    // buf from to_bytes doesn't contain null terminator
    write_buf(fd, s.to_bytes())
}
