fn get_semitone_ratio() -> f64 {
    2.0_f64.powf(1.0_f64 / 12.0_f64)
}

pub fn midi_to_freq(key: u8) -> f64 {
    let semitone_ratio = get_semitone_ratio();

    // We know that the frequency of A4 is 440 Hz
    let a4 = 69;

    let diff: i32 = key as i32 - a4;

    let mut freq = 440.0;

    if diff < 0 {
        freq /= (1.0 / semitone_ratio).powf(diff as f64);
    } else {
        freq *= semitone_ratio.powf(diff as f64);
    }

    return freq;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.1; // Hz tolerance for float comparison

    fn assert_freq(midi: u8, expected: f64) {
        let result = midi_to_freq(midi);
        assert!(
            (result - expected).abs() < EPSILON,
            "MIDI {}: expected {:.2} Hz, got {:.2} Hz",
            midi,
            expected,
            result
        );
    }

    #[test]
    fn test_a4_concert_pitch() {
        // The anchor: A4 = 440.00 Hz exactly
        assert_freq(69, 440.00);
    }

    #[test]
    fn test_middle_c() {
        // C4 (middle C) = 261.63 Hz
        assert_freq(60, 261.63);
    }

    #[test]
    fn test_lowest_piano_note() {
        // A0, lowest note on a standard piano
        assert_freq(21, 27.50);
    }

    #[test]
    fn test_highest_piano_note() {
        // C8, highest note on a standard piano
        assert_freq(108, 4186.01);
    }

    #[test]
    fn test_midi_zero() {
        assert_freq(0, 8.18);
    }

    #[test]
    fn test_midi_127() {
        assert_freq(127, 12543.85);
    }

    #[test]
    fn test_octave_doubling() {
        // Each octave doubles the frequency: A3=220, A4=440, A5=880
        assert_freq(57, 220.00);
        assert_freq(69, 440.00);
        assert_freq(81, 880.00);
    }

    #[test]
    fn test_semitone_spread() {
        // A handful of notes spread across the range
        assert_freq(48, 130.81); // C3
        assert_freq(64, 329.63); // E4
        assert_freq(76, 659.26); // E5
        assert_freq(93, 1760.00); // A6
    }
}
