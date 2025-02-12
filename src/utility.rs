use core::{ffi::c_char, slice};

pub unsafe fn c_str_to_str(s: *const c_char) -> &'static str {
    if s.is_null() {
        ""
    } else {
        let mut len = 0;
        while *s.offset(len) != 0 {
            len += 1;
        }
        let bytes = slice::from_raw_parts(s as _, len as usize);
        str::from_utf8_unchecked(bytes)
    }
}

// pub fn str_to_c_str(s: &str, buf: &mut [u8]) -> Option<*const c_char> {
//     if s.len() + 1 > buf.len() {
//         return None;
//     }
//     buf[..s.len()].copy_from_slice(s.as_bytes());
//     buf[s.len()] = 0;
//     Some(buf.as_ptr() as *const c_char)
// }
