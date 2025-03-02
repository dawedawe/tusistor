pub fn band_numeric_info(bands: usize, band_idx: usize, color: &rusistor::Color) -> String {
    match (bands, band_idx) {
        (3, i) | (4, i) if i <= 1 => {
            if i == 0 && *color == rusistor::Color::Black {
                " ".to_string()
            } else {
                color.as_digit().map_or(" ".to_string(), |s| s.to_string())
            }
        }
        (5, i) | (6, i) if i <= 2 => {
            if i == 0 && *color == rusistor::Color::Black {
                " ".to_string()
            } else {
                color.as_digit().map_or(" ".to_string(), |s| s.to_string())
            }
        }
        (3, 2) | (4, 2) | (5, 3) | (6, 3) => {
            format!("10^{}", color.as_digit_or_exponent())
        }
        (4, 3) | (5, 4) | (6, 4) => color
            .as_tolerance()
            .map_or("    ".to_string(), |s| format!("{:>4}", (s * 100.0))),
        (6, 5) => color
            .as_tcr()
            .map_or("   ".to_string(), |s| format!("{:>3}", s.to_string())),
        _ => "".to_string(),
    }
}

pub fn band_semantic_info(bands: usize, band_idx: usize) -> String {
    match (bands, band_idx) {
        (3, i) | (4, i) if i <= 1 => format!("Digit {}", band_idx + 1),
        (5, i) | (6, i) if i <= 2 => format!("Digit {}", band_idx + 1),
        (3, 2) | (4, 2) | (5, 3) | (6, 3) => "Multiplier".to_string(),
        (4, 3) | (5, 4) | (6, 4) => "Tolerance".to_string(),
        (6, 5) => "TCR".to_string(),
        _ => "".to_string(),
    }
}
