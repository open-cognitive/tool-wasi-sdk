//! # Open-Cognitive: Örnek WASM Aracı (Tool)
//! 
//! Bu kod, Bilişsel İşletim Sistemi'nden tamamen bağımsız olarak derlenir.

#[no_mangle]
pub extern "C" fn execute(input: i64) -> i64 {
    // Artık devasa sayılar çökmeyecek
    input * input
}