//! # Open-Cognitive: Örnek WASM Aracı (Tool)
//! 
//! Bu kod, Bilişsel İşletim Sistemi'nden tamamen bağımsız olarak derlenir.

#[no_mangle]
pub extern "C" fn execute(input: i32) -> i32 {
    // Ajanın eylemi: Gelen sayının karesini alır (Örn: 6 -> 36)
    input * input
}