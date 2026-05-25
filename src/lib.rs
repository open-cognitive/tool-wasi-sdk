//! # Open-Cognitive: WASM Araç SDK'sı

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;

#[no_mangle]
pub extern "C" fn execute(input: i64) -> i64 {
    input * input
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf); 
    ptr
}

#[no_mangle]
pub extern "C" fn process_text(ptr: *mut c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let input_str = c_str.to_str().unwrap_or("");
        
        let output_str = format!("(WASM MÜHÜRLÜ) İşlenen Metin: {}", input_str.to_uppercase());
        CString::new(output_str).unwrap().into_raw()
    }
}