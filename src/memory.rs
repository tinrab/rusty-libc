use core::{
    arch::asm,
    ffi::{c_int, c_size_t, c_void},
    ptr,
};

const SYSCALL_GETPAGESIZE: usize = 129;
const SYSCALL_MMAP: usize = 9;
const SYSCALL_MUNMAP: usize = 11;
const SYSCALL_MPROTECT: usize = 10;

const PROT_READ: c_int = 1;
const PROT_WRITE: c_int = 2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0 as *mut c_void; // (void *)(-1)

#[no_mangle]
pub unsafe extern "C" fn getpagesize() -> c_size_t {
    let pagesize: usize;
    asm!(
        "syscall",
        in("rax") SYSCALL_GETPAGESIZE,
        lateout("rax") pagesize,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
    );
    pagesize
}

#[no_mangle]
pub unsafe extern "C" fn mmap(
    addr: *mut c_void,
    len: c_size_t,
    prot: c_int,
    flags: c_int,
    fd: c_int,
    offset: isize,
) -> *mut c_void {
    let result: *mut c_void;
    asm!(
        "syscall",
        in("rax") SYSCALL_MMAP,
        in("rdi") addr,
        in("rsi") len,
        in("rdx") prot,
        in("r10") flags,
        in("r8") fd,
        in("r9") offset,
        lateout("rax") result,
        lateout("rcx") _, lateout("r11") _, // clobbered registers
        options(nostack, preserves_flags),
    );
    result
}

#[no_mangle]
pub unsafe extern "C" fn munmap(addr: *mut c_void, len: c_size_t) -> c_int {
    let result: c_int;
    asm!(
        "syscall",
        in("rax") SYSCALL_MUNMAP,
        in("rdi") addr,
        in("rsi") len,
        lateout("rax") result,
        lateout("rcx") _, lateout("r11") _, // clobbered registers
        options(nostack, preserves_flags),
    );
    result
}

#[no_mangle]
pub unsafe extern "C" fn mprotect(addr: *mut c_void, len: c_size_t, prot: c_int) -> c_int {
    let result: c_int;
    asm!(
        "syscall",
        in("rax") SYSCALL_MPROTECT,
        in("rdi") addr,
        in("rsi") len,
        in("rdx") prot,
        lateout("rax") result,
        lateout("rcx") _, lateout("r11") _, // clobbered registers
        options(nostack, preserves_flags),
    );
    result
}

/// A header placed before every allocated block.
#[repr(C)]
struct MallocHeader {
    size: usize,
}

#[no_mangle]
pub unsafe extern "C" fn malloc(size: c_size_t) -> *mut c_void {
    // `malloc` can either return NULL or a unique pointer
    let size = if size == 0 { 1 } else { size };

    let header_size = size_of::<MallocHeader>();
    // Calculate total size needed: header + requested payload
    let total_size = match size.checked_add(header_size) {
        Some(t) => t,
        None => return ptr::null_mut(),
    };

    let addr = mmap(
        ptr::null_mut(),
        total_size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    );
    if addr == MAP_FAILED {
        return ptr::null_mut();
    }

    // Write the header at the beginning of the allocated block
    let header_ptr = addr as *mut MallocHeader;
    (*header_ptr).size = total_size;

    // Return a pointer to the payload after the header
    header_ptr.add(1) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    // `free` can be called with NULL
    if ptr.is_null() {
        return;
    }

    let header_size = size_of::<MallocHeader>();
    let header_ptr = (ptr as *const u8).sub(header_size) as *mut MallocHeader;
    let total_size = (*header_ptr).size;

    munmap(header_ptr as *mut c_void, total_size);
}

#[no_mangle]
pub unsafe extern "C" fn calloc(nmemb: c_size_t, size: c_size_t) -> *mut c_void {
    // Calculate the total number of bytes
    let total = match nmemb.checked_mul(size) {
        Some(t) => t,
        None => return ptr::null_mut(),
    };

    let ptr = malloc(total);
    if !ptr.is_null() {
        // Zero out
        ptr::write_bytes(ptr, 0, total);
    }

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut c_void, size: c_size_t) -> *mut c_void {
    // `realloc`` behaves like `malloc` if the pointer is NULL
    if ptr.is_null() {
        return malloc(size);
    }

    // If size is 0, free the pointer and return NULL
    if size == 0 {
        free(ptr);
        return ptr::null_mut();
    }

    let header_size = size_of::<MallocHeader>();
    // Extract the header
    let prev_header = (ptr as *const u8).sub(header_size) as *mut MallocHeader;
    let prev_total = (*prev_header).size;
    let prev_payload = prev_total - header_size;

    // Ensure we allocate at least 1 byte if size is 0
    let new_size = size;
    let _new_total = match new_size.checked_add(header_size) {
        Some(t) => t,
        None => return ptr::null_mut(),
    };

    let new_ptr = malloc(new_size);
    if new_ptr.is_null() {
        return ptr::null_mut();
    }

    // Copy over the smaller of the previous payload and the new size.
    let copy_size = if prev_payload < new_size {
        prev_payload
    } else {
        new_size
    };
    ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
    free(ptr);

    new_ptr
}

#[no_mangle]
pub unsafe extern "C" fn reallocarray(
    ptr: *mut c_void,
    nmemb: c_size_t,
    size: c_size_t,
) -> *mut c_void {
    let total = match nmemb.checked_mul(size) {
        Some(t) => t,
        None => return ptr::null_mut(),
    };
    realloc(ptr, total)
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void {
    let mut d = dest as *mut u8;
    let mut s = src as *const u8;
    let mut c = n;

    // Use `rep movsb` instruction if count is large enough
    if c >= 8 {
        asm!(
            "rep movsb",
            inout("rdi") d => _,
            inout("rsi") s => _,
            inout("rcx") c => _,
            options(nostack, preserves_flags),
        );
    } else {
        // Handle remaining bytes if count is less than 8 or after the rep movsb loop
        while c > 0 {
            asm!(
                "movb (%rsi), %al",
                "movb %al, (%rdi)",
                in("rsi") s,
                in("rdi") d,
                options(nostack, preserves_flags, att_syntax),
            );
            d = d.offset(1);
            s = s.offset(1);
            c -= 1;
        }
    }

    d as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> c_int {
    let s1 = s1 as *const u8;
    let s2 = s2 as *const u8;
    for i in 0..n {
        let a = *s1.add(i);
        let b = *s2.add(i);
        if a != b {
            return a as c_int - b as c_int;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> c_int {
    memcmp(s1, s2, n)
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let mut d = s as *mut u8;
    let v = c as u8;
    let mut c = n;

    // Use `rep stosb` instruction for x86-64 if count is large enough
    if c >= 8 {
        asm!(
            "rep stosb",
            inout("rdi") d => _,
            in("al") v,
            inout("rcx") c => _,
            options(preserves_flags, nostack)
        );
    } else {
        // Handle remaining bytes if count is less than 8 or after the rep stosb loop
        while c > 0 {
            asm!(
                "movb %al, (%rdi)",
                in("al") v,
                in("rdi") d,
                options(preserves_flags, nostack, att_syntax)
            );
            d = d.offset(1);
            c -= 1;
        }
    }

    d as *mut c_void
}
