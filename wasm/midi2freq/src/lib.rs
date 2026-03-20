use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn midi_to_freq(key: u8) -> f64 {
    core::midi::midi_to_freq(key)
}
