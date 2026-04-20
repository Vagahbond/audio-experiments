use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn midi_to_freq(key: u8) -> f64 {
    core::midi::midi_to_freq(key)
}

#[wasm_bindgen]
pub fn freq_to_midi(freq: f64) -> u8 {
    core::midi::freq_to_midi(freq)
}

#[wasm_bindgen]
pub fn bend_amount(freq: f64) -> f64 {
    core::midi::bend_amount(freq)
}
