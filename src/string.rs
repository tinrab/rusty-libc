use core::ffi::{c_char, c_int, c_size_t};

#[no_mangle]
pub extern "C" fn strlen(s: *const c_char) -> c_size_t {
    unsafe {
        let mut len = 0;
        let mut p = s;
        while *p != 0 {
            len += 1;
            p = p.offset(1);
        }
        len
    }
}

#[no_mangle]
pub extern "C" fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int {
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;

        while *p1 != 0 && *p2 != 0 {
            let diff = *p1 - *p2;
            if diff != 0 {
                return diff as c_int;
            }
            p1 = p1.offset(1);
            p2 = p2.offset(1);
        }

        // If one string is prefix of another, or both are equal
        (*p1 - *p2) as c_int
    }
}

#[no_mangle]
pub extern "C" fn strncmp(s1: *const c_char, s2: *const c_char, n: c_size_t) -> c_int {
    unsafe {
        let mut p1 = s1;
        let mut p2 = s2;
        let mut count = 0;

        while count < n {
            let c1 = *p1;
            let c2 = *p2;

            if c1 == 0 || c2 == 0 {
                // Reached end of string
                return (c1 - c2) as c_int;
            }

            let diff = c1 - c2;
            if diff != 0 {
                return diff as c_int;
            }

            p1 = p1.offset(1);
            p2 = p2.offset(1);
            count += 1;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn toupper(c: c_int) -> c_int {
    if c >= 'a' as _ && c <= 'z' as _ {
        c - ('a' as c_int - 'A' as c_int)
    } else {
        c
    }
}

#[no_mangle]
pub extern "C" fn tolower(c: c_int) -> c_int {
    if c >= 'A' as _ && c <= 'Z' as _ {
        c + ('a' as c_int - 'A' as c_int)
    } else {
        c
    }
}
