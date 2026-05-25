//! # Open-Cognitive: WASM Araç SDK'sı

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::mem;

#[no_mangle]
pub extern "C" fn execute(input: i64) -> i64 { input * input }

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

#[no_mangle]
pub extern "C" fn generate_report(ptr: *mut c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let raw_data = c_str.to_str().unwrap_or("");
        let report = format!(
            "\n╭─────────────────────────────────╮\n│ 🚀 OPEN-COGNITIVE SYSTEM REPORT │\n├─────────────────────────────────┤\n│ 💻 Ortam: {:<21} │\n│ 🛡️ Güvenlik: Aktif (WASM)       │\n│ 🧠 Biliş: Sistem 1 & 2 Devrede  │\n╰─────────────────────────────────╯", 
            raw_data
        );
        CString::new(report).unwrap().into_raw()
    }
}

// YENİ ARAÇ: Host İşletim Sisteminden gelen ham dosya verisini güvenli UI'a çevirir
#[no_mangle]
pub extern "C" fn format_file(ptr: *mut c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        let content = c_str.to_str().unwrap_or("HATA: İçerik alınamadı.");
        
        let report = format!(
            "\n📄 DOSYA İÇERİĞİ (WASM Tarafından Denetlendi) 📄\n──────────────────────────────────────────────────\n{}\n──────────────────────────────────────────────────", 
            content.trim()
        );
        
        CString::new(report).unwrap().into_raw()
    }
}