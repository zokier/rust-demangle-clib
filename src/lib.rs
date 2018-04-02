extern crate rustc_demangle;

use std::os::raw::c_char;
use std::io::Write;

#[no_mangle]
pub extern fn RustDemangle(mangled: *const c_char, out: *mut u8, out_size: i64) -> i64 {
    unsafe {
    let mangled_str = std::ffi::CStr::from_ptr(mangled).to_str();
    if mangled_str.is_err() {
        return 0;
    }
    let mangled_str = mangled_str.unwrap();
    match rustc_demangle::try_demangle(mangled_str) {
        Ok(demangle) => {
            let mut out_slice = std::slice::from_raw_parts_mut(out, out_size as usize);
            match write!(out_slice, "{:#}\0", demangle) {
                Ok(_) => { return 1 },
                Err(_) => { return 0 }
            }
        },
        Err(_) => { return 0 }
    }
    }
}

