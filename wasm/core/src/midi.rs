fn get_semitone_ratio() -> f64 {
    2.0_f64.powf(1.0_f64 / 12.0_f64)
}

fn get_midi_0() -> f64 {
    /* find Middle C, three semitones above low A = 220 */
    let c5 = 220.0_f64 * get_semitone_ratio().powf(3.0);

    /* Note 0 is C, 5 octaves bel-ow Mlddle C */
    c5 * 0.5_f64.powf(5.0)
}

fn get_midi_0_string_length() -> f64 {
    // 660cm is the sounding length of a string playing C4.
    660.0_f64 * 2.0_f64.powf(5.0)
}

pub fn freq_to_midi(freq: f64) -> u8 {
    let semitone_ratio = get_semitone_ratio();

    let frac_midi = (freq / get_midi_0()).ln() / semitone_ratio.ln();

    frac_midi.round() as u8
}

pub fn midi_to_freq(key: u8) -> f64 {
    let semitone_ratio = get_semitone_ratio();

    // We know that the frequency of A4 is 440 Hz
    let a4 = 69;

    let diff: i32 = key as i32 - a4;

    let mut freq = 440.0;

    freq /= (1.0 / semitone_ratio).powf(diff as f64);

    return freq;
}

pub fn string_length2midi(length: f64) -> u8 {
    let semitone_ratio = get_semitone_ratio();

    let frac_midi = (get_midi_0_string_length() / length).ln() / semitone_ratio.ln();

    return  frac_midi.round() as u8;
}

pub fn string_midi2length(midi: u8) -> f64 {
    let semitone_ratio = get_semitone_ratio();

    let c4 = 60;

    let diff: i16 = midi as i16 - c4;

    let mut len = 660.0;

    len /= semitone_ratio.powf(diff as f64);

    len
}

pub fn string_length2freq(length: f64) -> f64 {
    let midi_0_len = get_midi_0_string_length();
    let midi_0_freq = get_midi_0();

    midi_0_freq * (midi_0_len/length)
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

    // --- Exact standard pitches ---

    #[test]
    fn test_a4_440hz() {
        // A4 = MIDI 69
        assert_eq!(freq_to_midi(440.0), 69);
    }

    #[test]
    fn test_middle_c_c4() {
        // C4 = MIDI 60
        assert_eq!(freq_to_midi(261.626), 60);
    }

    #[test]
    fn test_c0_is_midi_0() {
        // C0 is the reference note; should map to MIDI 0
        assert_eq!(freq_to_midi(get_midi_0()), 0);
    }

    #[test]
    fn test_a0() {
        // A0 = MIDI 21 (~27.5 Hz)
        assert_eq!(freq_to_midi(27.5), 21);
    }

    #[test]
    fn test_c8() {
        // C8 = MIDI 108 (~4186 Hz), highest note on a standard piano
        assert_eq!(freq_to_midi(4186.0), 108);
    }

    // --- Semitone intervals from A4 ---

    #[test]
    fn test_one_octave_up_from_a4() {
        // A5 = MIDI 81, double the frequency
        assert_eq!(freq_to_midi(880.0), 81);
    }

    #[test]
    fn test_one_octave_down_from_a4() {
        // A3 = MIDI 57, half the frequency
        assert_eq!(freq_to_midi(220.0), 57);
    }

    // --- Rounding behaviour ---

    #[test]
    fn test_rounds_to_nearest_below() {
        // Slightly flat of A4 still rounds to 69
        assert_eq!(freq_to_midi(438.0), 69);
    }

    #[test]
    fn test_rounds_to_nearest_above() {
        // Slightly sharp of A4 still rounds to 69
        assert_eq!(freq_to_midi(442.0), 69);
    }

    // --- Frequencies that should NOT match a given MIDI note ---

    #[test]
    fn test_not_a4_when_too_flat() {
        // More than a quarter-tone flat of A4 — should resolve to G#4 (68)
        let g_sharp_4 = 415.305;
        assert_ne!(freq_to_midi(g_sharp_4 + 1.0), 69);
        assert_eq!(freq_to_midi(g_sharp_4 + 1.0), 68);
    }

    #[test]
    fn test_not_a4_when_too_sharp() {
        // More than a quarter-tone sharp of A4 — should resolve to A#4 (70)
        let a_sharp_4 = 466.164;
        assert_ne!(freq_to_midi(a_sharp_4 - 1.0), 69);
        assert_eq!(freq_to_midi(a_sharp_4 - 1.0), 70);
    }

    #[test]
    fn test_not_middle_c_when_sharp() {
        // C#4 (MIDI 61) should not map to C4 (60)
        assert_ne!(freq_to_midi(277.183), 60);
        assert_eq!(freq_to_midi(277.183), 61);
    }

    #[test]
    fn test_not_middle_c_when_flat() {
        // B3 (MIDI 59) should not map to C4 (60)
        assert_ne!(freq_to_midi(246.942), 60);
        assert_eq!(freq_to_midi(246.942), 59);
    }

    #[test]
    fn test_not_c0_when_one_semitone_up() {
        // C#0 (MIDI 1) should not map to MIDI 0
        let c_sharp_0 = get_midi_0() * get_semitone_ratio();
        assert_ne!(freq_to_midi(c_sharp_0), 0);
        assert_eq!(freq_to_midi(c_sharp_0), 1);
    }

    #[test]
    fn test_adjacent_notes_are_distinct() {
        // Every semitone from MIDI 60–72 should map to a unique value —
        // no two adjacent exact frequencies should collapse to the same note
        let c4_midi = 60u8;
        let c4_freq = get_midi_0() * get_semitone_ratio().powi(c4_midi as i32);

        for semitone in 0..12 {
            let freq = c4_freq * get_semitone_ratio().powi(semitone);
            let result = freq_to_midi(freq);
            assert_eq!(
                result,
                c4_midi + semitone as u8,
                "Expected MIDI {} for semitone offset {}, got {}",
                c4_midi + semitone as u8,
                semitone,
                result
            );
        }
    }

    #[test]
    fn test_rounds_up_at_boundary() {
        // Exactly halfway between A4 (69) and A#4 (70) should round to 70
        let a4 = 440.0_f64;
        let a_sharp_4 = a4 * get_semitone_ratio();
        let midpoint = (a4 + a_sharp_4) / 2.0;
        assert_eq!(freq_to_midi(midpoint), 70);
    }

    // --- string_midi2length tests ---

    #[test]
    fn test_string_midi2length_c4() {
        let len = string_midi2length(60);
        assert!(
            (len - 660.0).abs() < EPSILON,
            "expected 660.0 and got {}",
            len
        );
    }

    #[test]
    fn test_string_midi2length_c5() {
        let len = string_midi2length(72);
        assert!(
            (len - 330.0).abs() < EPSILON,
            "expected 330.0 and got {}",
            len
        );
    }

    #[test]
    fn test_string_midi2length_c3() {
        let len = string_midi2length(48);
        assert!(
            (len - 1320.0).abs() < EPSILON,
            "expected 1320.0 and got {}",
            len
        );
    }

    // --- string_length2freq tests ---

    #[test]
    fn test_string_length2freq_c4() {
        // C4 = 660 cm → 261.63 Hz
        let freq = string_length2freq(660.0);
        assert!(
            (freq - 261.626).abs() < EPSILON,
            "expected ~261.63 Hz for C4 (660 cm), got {:.4}",
            freq
        );
    }

    #[test]
    fn test_string_length2freq_c3() {
        // C3 = 1320 cm (double C4) → 130.81 Hz (half C4)
        let freq = string_length2freq(1320.0);
        assert!(
            (freq - 130.813).abs() < EPSILON,
            "expected ~130.81 Hz for C3 (1320 cm), got {:.4}",
            freq
        );
    }

    #[test]
    fn test_string_length2freq_c5() {
        // C5 = 330 cm (half C4) → 523.25 Hz (double C4)
        let freq = string_length2freq(330.0);
        assert!(
            (freq - 523.251).abs() < EPSILON,
            "expected ~523.25 Hz for C5 (330 cm), got {:.4}",
            freq
        );
    }

    #[test]
    fn test_string_length2freq_c0() {
        // C0 = 21120 cm (the reference length) → 8.176 Hz
        let freq = string_length2freq(21120.0);
        assert!(
            (freq - 8.176).abs() < EPSILON,
            "expected ~8.176 Hz for C0 (21120 cm), got {:.4}",
            freq
        );
    }

    #[test]
    fn test_string_length2freq_a4() {
        // A4 = ~392.438 cm → 440.0 Hz
        let semitone_ratio = get_semitone_ratio();
        let a4_len = 660.0 / semitone_ratio.powi(9); // 9 semitones above C4
        let freq = string_length2freq(a4_len);
        assert!(
            (freq - 440.0).abs() < EPSILON,
            "expected 440.0 Hz for A4 ({:.4} cm), got {:.4}",
            a4_len,
            freq
        );
    }

    #[test]
    fn test_string_length2freq_g4() {
        // G4 = ~440.497 cm → 391.995 Hz (perfect fifth above C4)
        let semitone_ratio = get_semitone_ratio();
        let g4_len = 660.0 / semitone_ratio.powi(7); // 7 semitones above C4
        let freq = string_length2freq(g4_len);
        assert!(
            (freq - 391.995).abs() < EPSILON,
            "expected ~391.995 Hz for G4 ({:.4} cm), got {:.4}",
            g4_len,
            freq
        );
    }

    #[test]
    fn test_string_length2freq_longer_is_lower() {
        // Fundamental physics: a longer string always produces a lower frequency
        let freq_short = string_length2freq(330.0);
        let freq_long = string_length2freq(1320.0);
        assert!(
            freq_short > freq_long,
            "shorter string ({} Hz) should have higher freq than longer string ({} Hz)",
            freq_short,
            freq_long
        );
    }

    #[test]
    fn test_string_length2freq_doubling_halves_freq() {
        // Doubling the string length should halve the frequency (one octave down)
        let freq_base = string_length2freq(660.0);
        let freq_double = string_length2freq(1320.0);
        assert!(
            (freq_double - freq_base / 2.0).abs() < EPSILON,
            "doubling length should halve frequency: expected {:.4}, got {:.4}",
            freq_base / 2.0,
            freq_double
        );
    }

    #[test]
    fn test_string_length2freq_roundtrip() {
        // For notes in a 3-octave range, string_length2freq(string_midi2length(n))
        // should equal midi_to_freq(n).
        for midi in 36u8..=84 {
            let len = string_midi2length(midi);
            let freq_from_len = string_length2freq(len);
            let freq_from_midi = midi_to_freq(midi);
            assert!(
                (freq_from_len - freq_from_midi).abs() < EPSILON,
                "round-trip failed for MIDI {}: length {:.4} cm -> {:.4} Hz, expected {:.4} Hz",
                midi,
                len,
                freq_from_len,
                freq_from_midi
            );
        }
    }

    // --- string_length2midi tests ---

    #[test]
    fn test_string_length2midi_660_is_high() {
        let result = string_length2midi(660.0);
        assert_eq!(result, 60, "expected 60 and got {}", result);
    }

    #[test]
    fn test_string_length2midi_doubling() {
        let res = string_length2midi(1320.0);
        assert_eq!(res, 48, "expected 48 and got {}", res);
    }

    #[test]
    fn test_string_length2midi_halfing() {
        let res = string_length2midi(330.0);
        assert_eq!(res, 72, "expected 72 and got {}", res);
    }

    // --- string_length2midi: semitone steps around C4 ---

    #[test]
    fn test_string_length2midi_one_semitone_shorter_is_higher() {
        // C#4 = MIDI 61, string length ~622.957 cm (shorter than C4's 660 cm)
        let res = string_length2midi(622.957);
        assert_eq!(res, 61, "expected 61 (C#4) and got {}", res);
    }

    #[test]
    fn test_string_length2midi_one_semitone_longer_is_lower() {
        // B3 = MIDI 59, string length ~699.246 cm (longer than C4's 660 cm)
        let res = string_length2midi(699.246);
        assert_eq!(res, 59, "expected 59 (B3) and got {}", res);
    }

    // --- string_length2midi: named notes across multiple octaves ---

    #[test]
    fn test_string_length2midi_c1() {
        // C1 = MIDI 24, string length 5280.0 cm
        let res = string_length2midi(5280.0);
        assert_eq!(res, 24, "expected 24 (C1) and got {}", res);
    }

    #[test]
    fn test_string_length2midi_c2() {
        // C2 = MIDI 36, string length 2640.0 cm
        let res = string_length2midi(2640.0);
        assert_eq!(res, 36, "expected 36 (C2) and got {}", res);
    }

    // --- string_length2midi: non-octave intervals ---

    #[test]
    fn test_string_length2midi_g4() {
        // G4 = MIDI 67, a perfect fifth above C4 (~440.497 cm)
        let res = string_length2midi(440.497);
        assert_eq!(res, 67, "expected 67 (G4) and got {}", res);
    }

    #[test]
    fn test_string_length2midi_f3() {
        // F3 = MIDI 53, a perfect fifth below C4 (~988.883 cm)
        let res = string_length2midi(988.883);
        assert_eq!(res, 53, "expected 53 (F3) and got {}", res);
    }
}
