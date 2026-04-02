use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn string_length2midi(length: f64) -> u8 {
    core::midi::string_length2midi(length)
}

#[wasm_bindgen]
pub fn string_midi2length(midi: u8) -> f64 {
    core::midi::string_midi2length(midi)
}

#[wasm_bindgen]
pub fn string_length2freq(length: f64) -> f64 {
    core::midi::string_length2freq(length)
} 


