use core::panic;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Color {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
    Gold,
    Silver,
    Pink,
}

impl Color {
    fn as_digit_or_exponent(&self) -> f64 {
        match self {
            Color::Black => 0.0,
            Color::Brown => 1.0,
            Color::Red => 2.0,
            Color::Orange => 3.0,
            Color::Yellow => 4.0,
            Color::Green => 5.0,
            Color::Blue => 6.0,
            Color::Violet => 7.0,
            Color::Grey => 8.0,
            Color::White => 9.0,
            Color::Gold => -1.0,
            Color::Silver => -2.0,
            Color::Pink => -3.0,
        }
    }

    fn as_tolerance(&self) -> f64 {
        match self {
            Color::Black => panic!("invalid tolerance color"),
            Color::Brown => 0.01,
            Color::Red => 0.02,
            Color::Orange => 0.0005,
            Color::Yellow => 0.0002,
            Color::Green => 0.005,
            Color::Blue => 0.0025,
            Color::Violet => 0.001,
            Color::Grey => 0.0001,
            Color::White => panic!("invalid tolerance color"),
            Color::Gold => 0.05,
            Color::Silver => 0.1,
            Color::Pink => panic!("invalid tolerance color"),
        }
    }

    fn as_tcr(&self) -> u32 {
        match self {
            Color::Black => 250,
            Color::Brown => 100,
            Color::Red => 50,
            Color::Orange => 15,
            Color::Yellow => 25,
            Color::Green => 20,
            Color::Blue => 10,
            Color::Violet => 5,
            Color::Grey => 1,
            Color::White => panic!("invalid tcr color"),
            Color::Gold => panic!("invalid tcr color"),
            Color::Silver => panic!("invalid tcr color"),
            Color::Pink => panic!("invalid tcr color"),
        }
    }

    fn from_tolerance(tolerance: f64) -> Color {
        match tolerance {
            1.0 => Color::Brown,
            2.0 => Color::Red,
            0.05 => Color::Orange,
            0.02 => Color::Yellow,
            0.5 => Color::Green,
            0.25 => Color::Blue,
            0.1 => Color::Violet,
            0.01 => Color::Grey,
            5.0 => Color::Gold,
            10.0 => Color::Silver,
            _ => panic!("invalid tolerance value {tolerance}"),
        }
    }

    fn from_tcr(tcr: u32) -> Color {
        match tcr {
            250 => Color::Black,
            100 => Color::Brown,
            50 => Color::Red,
            15 => Color::Orange,
            25 => Color::Yellow,
            20 => Color::Green,
            10 => Color::Blue,
            5 => Color::Violet,
            1 => Color::Grey,
            _ => panic!("invalid tcr value"),
        }
    }
}

impl From<i32> for Color {
    fn from(value: i32) -> Self {
        match value {
            0 => Color::Black,
            1 => Color::Brown,
            2 => Color::Red,
            3 => Color::Orange,
            4 => Color::Yellow,
            5 => Color::Green,
            6 => Color::Blue,
            7 => Color::Violet,
            8 => Color::Grey,
            9 => Color::White,
            -1 => Color::Gold,
            -2 => Color::Silver,
            -3 => Color::Pink,
            _ => panic!("invalid value {} given to Color::from", value),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Resistor {
    ZeroOhm,
    ThreeBand {
        band1: Color,
        band2: Color,
        band3: Color,
    },
    FourBand {
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
    },
    FiveBand {
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
        band5: Color,
    },
    SixBand {
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
        band5: Color,
        band6: Color,
    },
}

impl Resistor {
    fn is_valid_color_in_band(color: &Color, band_position: usize, band_count: usize) -> bool {
        let mut valid_configs = HashSet::new();
        // zero-ohm resistor
        valid_configs.insert((&Color::Black, 1, 1));
        // band 1 in 3-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 1, 3));
        }
        // band 2 in 3-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 2, 3));
        }
        // band 3 in 3-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
            &Color::Gold,
            &Color::Silver,
            &Color::Pink,
        ] {
            valid_configs.insert((c, 3, 3));
        }
        // band 1 in 4-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 1, 4));
        }
        // band 2 in 4-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 2, 4));
        }
        // band 3 in 4-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
            &Color::Gold,
            &Color::Silver,
            &Color::Pink,
        ] {
            valid_configs.insert((c, 3, 4));
        }
        // band 4 in 4-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::Gold,
            &Color::Silver,
        ] {
            valid_configs.insert((c, 4, 4));
        }
        // band 1 in 5-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 1, 5));
        }
        // band 2 in 5-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 2, 5));
        }
        // band 3 in 5-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 3, 5));
        }
        // band 4 in 5-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
            &Color::Gold,
            &Color::Silver,
            &Color::Pink,
        ] {
            valid_configs.insert((c, 4, 5));
        }
        // band 5 in 5-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::Gold,
            &Color::Silver,
        ] {
            valid_configs.insert((c, 5, 5));
        }
        // band 1 in 6-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 1, 6));
        }
        // band 2 in 6-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 2, 6));
        }
        // band 3 in 6-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
        ] {
            valid_configs.insert((c, 3, 6));
        }
        // band 4 in 6-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::White,
            &Color::Gold,
            &Color::Silver,
            &Color::Pink,
        ] {
            valid_configs.insert((c, 4, 6));
        }
        // band 5 in 6-band resistor
        for c in [
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
            &Color::Gold,
            &Color::Silver,
        ] {
            valid_configs.insert((c, 5, 6));
        }
        // band 6 in 6-band resistor
        for c in [
            &Color::Black,
            &Color::Brown,
            &Color::Red,
            &Color::Orange,
            &Color::Yellow,
            &Color::Green,
            &Color::Blue,
            &Color::Violet,
            &Color::Grey,
        ] {
            valid_configs.insert((c, 6, 6));
        }

        valid_configs.contains(&(color, band_position, band_count))
    }

    fn validate_tolerance(tolerance: &Option<f64>) -> Result<Option<f64>, String> {
        let valid_values = [1.0, 2.0, 0.05, 0.02, 0.5, 0.25, 0.1, 0.01, 5.0, 10.0];
        match tolerance {
            Some(tolerance) => {
                if valid_values.contains(tolerance) {
                    Result::Ok(Some(*tolerance))
                } else {
                    Result::Err(String::from("not a valid tolerance value"))
                }
            }
            None => Result::Ok(None),
        }
    }

    fn validate_tcr(tcr: &Option<u32>) -> Result<Option<u32>, String> {
        let valid_values = [250, 100, 50, 15, 25, 20, 10, 5, 1];
        match tcr {
            Some(tcr) => {
                if valid_values.contains(tcr) {
                    Result::Ok(Some(*tcr))
                } else {
                    Result::Err(String::from("not a valid tcr value"))
                }
            }
            None => Result::Ok(None),
        }
    }

    pub fn try_create(bands: Vec<Color>) -> Result<Resistor, String> {
        match bands.len() {
            1 => Resistor::try_create_1_band(bands[0].clone()),
            3 => Resistor::try_create_3_band(bands[0].clone(), bands[1].clone(), bands[2].clone()),
            4 => Resistor::try_create_4_band(
                bands[0].clone(),
                bands[1].clone(),
                bands[2].clone(),
                bands[3].clone(),
            ),
            5 => Resistor::try_create_5_band(
                bands[0].clone(),
                bands[1].clone(),
                bands[2].clone(),
                bands[3].clone(),
                bands[4].clone(),
            ),
            6 => Resistor::try_create_6_band(
                bands[0].clone(),
                bands[1].clone(),
                bands[2].clone(),
                bands[3].clone(),
                bands[4].clone(),
                bands[5].clone(),
            ),
            _ => Err(String::from("This is not a valid transistor configuration")),
        }
    }

    fn try_create_1_band(band: Color) -> Result<Resistor, String> {
        if Resistor::is_valid_color_in_band(&band, 1, 1) {
            Ok(Resistor::ZeroOhm)
        } else {
            Err(String::from(
                "In a single band resistor, only a black band is valid.",
            ))
        }
    }

    fn try_create_3_band(band1: Color, band2: Color, band3: Color) -> Result<Resistor, String> {
        if Resistor::is_valid_color_in_band(&band1, 1, 3)
            && Resistor::is_valid_color_in_band(&band2, 2, 3)
            && Resistor::is_valid_color_in_band(&band3, 3, 3)
        {
            Ok(Resistor::ThreeBand {
                band1,
                band2,
                band3,
            })
        } else {
            Err(String::from("no valid 3-band resistor"))
        }
    }

    fn try_create_4_band(
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
    ) -> Result<Resistor, String> {
        if Resistor::is_valid_color_in_band(&band1, 1, 4)
            && Resistor::is_valid_color_in_band(&band2, 2, 4)
            && Resistor::is_valid_color_in_band(&band3, 3, 4)
            && Resistor::is_valid_color_in_band(&band4, 4, 4)
        {
            Ok(Resistor::FourBand {
                band1,
                band2,
                band3,
                band4,
            })
        } else {
            Err(String::from("no valid 4-band resistor"))
        }
    }

    fn try_create_5_band(
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
        band5: Color,
    ) -> Result<Resistor, String> {
        if Resistor::is_valid_color_in_band(&band1, 1, 5)
            && Resistor::is_valid_color_in_band(&band2, 2, 5)
            && Resistor::is_valid_color_in_band(&band3, 3, 5)
            && Resistor::is_valid_color_in_band(&band4, 4, 5)
            && Resistor::is_valid_color_in_band(&band5, 5, 5)
        {
            Ok(Resistor::FiveBand {
                band1,
                band2,
                band3,
                band4,
                band5,
            })
        } else {
            Err(String::from("no valid 4-band resistor"))
        }
    }

    fn try_create_6_band(
        band1: Color,
        band2: Color,
        band3: Color,
        band4: Color,
        band5: Color,
        band6: Color,
    ) -> Result<Resistor, String> {
        if Resistor::is_valid_color_in_band(&band1, 1, 6)
            && Resistor::is_valid_color_in_band(&band2, 2, 6)
            && Resistor::is_valid_color_in_band(&band3, 3, 6)
            && Resistor::is_valid_color_in_band(&band4, 4, 6)
            && Resistor::is_valid_color_in_band(&band5, 5, 6)
            && Resistor::is_valid_color_in_band(&band6, 6, 6)
        {
            Ok(Resistor::SixBand {
                band1,
                band2,
                band3,
                band4,
                band5,
                band6,
            })
        } else {
            Err(String::from("no valid 4-band resistor"))
        }
    }

    pub fn calc(&self) -> (f64, f64, f64, Option<u32>) {
        match &self {
            Resistor::ZeroOhm => (0.0, 0.0, 0.0, None),
            Resistor::ThreeBand {
                band1,
                band2,
                band3,
            } => {
                let digit1 = band1.as_digit_or_exponent();
                let digit2 = band2.as_digit_or_exponent();
                let exponent = band3.as_digit_or_exponent();
                let tolerance = 0.2;
                let multiplier = 10.0f64.powf(exponent);
                let ohm = (digit1 * 10.0 + digit2) * multiplier;
                let tolerance_ohm = ohm * tolerance;
                let min = ohm - tolerance_ohm;
                let max = ohm + tolerance_ohm;
                (ohm, min, max, None)
            }
            Resistor::FourBand {
                band1,
                band2,
                band3,
                band4,
            } => {
                let digit1 = band1.as_digit_or_exponent();
                let digit2 = band2.as_digit_or_exponent();
                let exponent = band3.as_digit_or_exponent();
                let tolerance = band4.as_tolerance();
                let multiplier = 10.0f64.powf(exponent);
                let ohm = (digit1 * 10.0 + digit2) * multiplier;
                let tolerance_ohm = ohm * tolerance;
                let min = ohm - tolerance_ohm;
                let max = ohm + tolerance_ohm;
                (ohm, min, max, None)
            }
            Resistor::FiveBand {
                band1,
                band2,
                band3,
                band4,
                band5,
            } => {
                let digit1 = band1.as_digit_or_exponent();
                let digit2 = band2.as_digit_or_exponent();
                let digit3 = band3.as_digit_or_exponent();
                let exponent = band4.as_digit_or_exponent();
                let tolerance = band5.as_tolerance();
                let multiplier = 10.0f64.powf(exponent);
                let ohm = (digit1 * 100.0 + digit2 * 10.0 + digit3) * multiplier;
                let tolerance_ohm = ohm * tolerance;
                let min = ohm - tolerance_ohm;
                let max = ohm + tolerance_ohm;
                (ohm, min, max, None)
            }
            Resistor::SixBand {
                band1,
                band2,
                band3,
                band4,
                band5,
                band6,
            } => {
                let digit1 = band1.as_digit_or_exponent();
                let digit2 = band2.as_digit_or_exponent();
                let digit3 = band3.as_digit_or_exponent();
                let exponent = band4.as_digit_or_exponent();
                let tolerance = band5.as_tolerance();
                let tcr = band6.as_tcr();
                let multiplier = 10.0f64.powf(exponent);
                let ohm = (digit1 * 100.0 + digit2 * 10.0 + digit3) * multiplier;
                let tolerance_ohm = ohm * tolerance;
                let min = ohm - tolerance_ohm;
                let max = ohm + tolerance_ohm;
                (ohm, min, max, Some(tcr))
            }
        }
    }

    pub fn determine_digits_and_exponent(ohm: f64) -> Result<(Vec<u32>, i32), String> {
        let mut exponent = 0i32;
        let mut s = ohm.to_string();

        let s2 = s.replace(".", "");
        let s2 = s2.trim_matches(|c| c == '0');
        if s2.len() > 3 {
            Err(format!(
                "number {} can't be represented in two three bands",
                ohm
            ))
        } else {
            // push left if needed
            let mut dot_idx = s.find('.');
            while dot_idx.is_some() {
                let idx = dot_idx.unwrap();
                exponent -= 1;
                s.remove(idx);
                s.insert(idx + 1, '.');
                let n = s.parse::<f64>().unwrap();
                s = n.to_string();
                dot_idx = s.find('.');
            }

            // push single digit left if not zero-ohm resistor
            if s.len() == 1 && s != "0" {
                s.insert(1, '0');
                exponent -= 1;
            }

            // push right if needed
            let exceeds_2_band_range = ohm > 99000000000.0;
            while s.len() > 3
                || (!exceeds_2_band_range && s.len() == 3 && s.chars().nth(2) == Some('0'))
            {
                exponent += 1;
                s.truncate(s.len() - 1);
            }

            let digits: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
            Ok((digits, exponent))
        }
    }

    pub fn determine(
        resistance: f64,
        tolerance: Option<f64>,
        tcr: Option<u32>,
    ) -> Result<Resistor, String> {
        let digits = Resistor::determine_digits_and_exponent(resistance);
        let tolerance = Resistor::validate_tolerance(&tolerance);
        let tcr = Resistor::validate_tcr(&tcr);

        match (digits, tolerance, tcr) {
            (Ok((digits, 0)), Ok(None), Ok(None)) if digits.len() == 1 => {
                Resistor::try_create_1_band(Color::from(digits[0] as i32))
            }
            (Ok((digits, e)), Ok(None), Ok(None)) if digits.len() == 2 => {
                Resistor::try_create_3_band(
                    Color::from(digits[0] as i32),
                    Color::from(digits[1] as i32),
                    Color::from(e),
                )
            }
            (Ok((digits, e)), Ok(Some(tol)), Ok(None)) if digits.len() == 2 => {
                Resistor::try_create_4_band(
                    Color::from(digits[0] as i32),
                    Color::from(digits[1] as i32),
                    Color::from(e),
                    Color::from_tolerance(tol),
                )
            }
            (Ok((digits, e)), Ok(Some(tol)), Ok(Some(tcr))) if digits.len() == 2 => {
                Resistor::try_create_6_band(
                    Color::from(digits[0] as i32),
                    Color::from(digits[1] as i32),
                    Color::from(0),
                    Color::from(e - 1),
                    Color::from_tolerance(tol),
                    Color::from_tcr(tcr),
                )
            }
            (Ok((digits, e)), Ok(Some(tol)), Ok(None)) if digits.len() == 3 => {
                Resistor::try_create_5_band(
                    Color::from(digits[0] as i32),
                    Color::from(digits[1] as i32),
                    Color::from(digits[2] as i32),
                    Color::from(e),
                    Color::from_tolerance(tol),
                )
            }
            (Ok((digits, e)), Ok(Some(tol)), Ok(Some(tcr))) if digits.len() == 3 => {
                Resistor::try_create_6_band(
                    Color::from(digits[0] as i32),
                    Color::from(digits[1] as i32),
                    Color::from(digits[2] as i32),
                    Color::from(e),
                    Color::from_tolerance(tol),
                    Color::from_tcr(tcr),
                )
            }
            (Ok((digits, _e)), Ok(None), Ok(None)) if digits.len() == 3 => {
                Err(String::from("A 3-digit resistor needs a tolerance."))
            }
            (Ok(_), Err(e), _) => Err(e),
            _ => Err(String::from("Not a representable resistance value.")),
        }
    }

    pub fn bands(&self) -> Vec<&Color> {
        match self {
            Resistor::ZeroOhm => vec![&Color::Black],
            Resistor::ThreeBand {
                band1,
                band2,
                band3,
            } => vec![band1, band2, band3],
            Resistor::FourBand {
                band1,
                band2,
                band3,
                band4,
            } => vec![band1, band2, band3, band4],
            Resistor::FiveBand {
                band1,
                band2,
                band3,
                band4,
                band5,
            } => vec![band1, band2, band3, band4, band5],
            Resistor::SixBand {
                band1,
                band2,
                band3,
                band4,
                band5,
                band6,
            } => vec![band1, band2, band3, band4, band5, band6],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn try_create_valid_zeroohm_resistor() {
        let bands = vec![Color::Black];
        let r = Resistor::try_create(bands);
        assert_eq!(r, Ok(Resistor::ZeroOhm));
    }

    #[test]
    pub fn try_create_invalid_1_band_resistor() {
        let bands = vec![Color::Blue];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn try_create_invalid_2_band_resistor() {
        let bands = vec![Color::Gold, Color::Grey];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn try_create_valid_3_band_resistor() {
        let bands = vec![Color::Blue, Color::Brown, Color::Pink];
        let r = Resistor::try_create(bands);
        assert!(r.is_ok());
    }

    #[test]
    pub fn try_create_invalid_3_band_resistor() {
        let bands = vec![Color::Black, Color::Brown, Color::Pink];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn try_create_valid_4_band_resistor() {
        let bands = vec![Color::Blue, Color::Brown, Color::Pink, Color::Silver];
        let r = Resistor::try_create(bands);
        assert!(r.is_ok());
    }

    #[test]
    pub fn try_create_invalid_4_band_resistor() {
        let bands = vec![Color::Blue, Color::Brown, Color::Pink, Color::Black];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn try_create_valid_5_band_resistor() {
        let bands = vec![
            Color::Blue,
            Color::Brown,
            Color::White,
            Color::Silver,
            Color::Silver,
        ];
        let r = Resistor::try_create(bands);
        assert!(r.is_ok());
    }

    #[test]
    pub fn try_create_invalid_5_band_resistor() {
        let bands = vec![
            Color::Blue,
            Color::Brown,
            Color::Pink,
            Color::Black,
            Color::Black,
        ];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn try_create_valid_6_band_resistor() {
        let bands = vec![
            Color::Blue,
            Color::Brown,
            Color::White,
            Color::Silver,
            Color::Silver,
            Color::Black,
        ];
        let r = Resistor::try_create(bands);
        assert!(r.is_ok());
    }

    #[test]
    pub fn try_create_invalid_6_band_resistor() {
        let bands = vec![
            Color::Blue,
            Color::Brown,
            Color::Grey,
            Color::Black,
            Color::Silver,
            Color::White,
        ];
        let r = Resistor::try_create(bands);
        assert!(r.is_err());
    }

    #[test]
    pub fn calc_zeroohm_resistor() {
        let r = Resistor::try_create_1_band(Color::Black).unwrap();
        let o = r.calc();
        assert_eq!(o, (0.0, 0.0, 0.0, None))
    }

    #[test]
    pub fn calc_3_band_resistor() {
        let r = Resistor::try_create_3_band(Color::Red, Color::Black, Color::Pink);
        let o = r.unwrap().calc();
        assert_eq!(o, (0.02, 0.016, 0.024, None))
    }

    #[test]
    pub fn calc_4_band_resistor_1() {
        let r = Resistor::try_create_4_band(Color::Red, Color::Red, Color::Orange, Color::Gold)
            .unwrap();
        let o = r.calc();
        assert_eq!(o, (22000.0, 20900.0, 23100.0, None))
    }

    #[test]
    pub fn calc_4_band_resistor_2() {
        let r =
            Resistor::try_create_4_band(Color::Yellow, Color::Violet, Color::Brown, Color::Gold)
                .unwrap();
        let o = r.calc();
        assert_eq!(o, (470.0, 446.5, 493.5, None))
    }

    #[test]
    pub fn calc_4_band_resistor_3() {
        let r = Resistor::try_create_4_band(Color::Blue, Color::Grey, Color::Black, Color::Orange)
            .unwrap();
        let o = r.calc();
        assert_eq!(o, (68.0, 67.966, 68.034, None))
    }

    #[test]
    pub fn calc_5_band_resistor() {
        let r = Resistor::try_create_5_band(
            Color::Green,
            Color::Blue,
            Color::Black,
            Color::Black,
            Color::Brown,
        )
        .unwrap();
        let o = r.calc();
        assert_eq!(o, (560.0, 554.4, 565.6, None))
    }

    #[test]
    pub fn calc_6_band_resistor() {
        let r = Resistor::try_create_6_band(
            Color::Green,
            Color::Blue,
            Color::Black,
            Color::Black,
            Color::Brown,
            Color::Grey,
        )
        .unwrap();
        let o = r.calc();
        assert_eq!(o, (560.0, 554.4, 565.6, Some(1)))
    }

    #[test]
    pub fn determine_resistor() {
        let r = Resistor::determine(200.0, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Red, &Color::Black, &Color::Brown]);

        let r = Resistor::determine(210.0, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Red, &Color::Brown, &Color::Brown]);

        let r = Resistor::determine(20.0, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Red, &Color::Black, &Color::Black]);

        let r = Resistor::determine(11.0, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Brown, &Color::Brown, &Color::Black]);

        let r = Resistor::determine(1.0, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Brown, &Color::Black, &Color::Gold]);

        let r = Resistor::determine(9.8, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::White, &Color::Grey, &Color::Gold]);

        let r = Resistor::determine(0.8, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Grey, &Color::Black, &Color::Silver]);

        let r = Resistor::determine(0.59, None, None).unwrap();
        assert_eq!(
            r.bands(),
            vec![&Color::Green, &Color::White, &Color::Silver]
        );

        let r = Resistor::determine(0.1, None, None).unwrap();
        assert_eq!(
            r.bands(),
            vec![&Color::Brown, &Color::Black, &Color::Silver]
        );

        let r = Resistor::determine(0.01, None, None).unwrap();
        assert_eq!(r.bands(), vec![&Color::Brown, &Color::Black, &Color::Pink]);

        let r = Resistor::determine(0.047, None, None).unwrap();
        assert_eq!(
            r.bands(),
            vec![&Color::Yellow, &Color::Violet, &Color::Pink]
        );

        let r = Resistor::determine(0.123, Some(0.5), None).unwrap();
        assert_eq!(
            r.bands(),
            vec![
                &Color::Brown,
                &Color::Red,
                &Color::Orange,
                &Color::Pink,
                &Color::Green
            ]
        );

        let r = Resistor::determine(0.123, Some(0.5), Some(50)).unwrap();
        assert_eq!(
            r.bands(),
            vec![
                &Color::Brown,
                &Color::Red,
                &Color::Orange,
                &Color::Pink,
                &Color::Green,
                &Color::Red
            ]
        );

        let r = Resistor::determine(0.012, Some(10.0), None).unwrap();
        assert_eq!(
            r.bands(),
            vec![&Color::Brown, &Color::Red, &Color::Pink, &Color::Silver]
        );

        let r = Resistor::determine(54.0, Some(10.0), Some(5)).unwrap();
        assert_eq!(
            r.bands(),
            vec![
                &Color::Green,
                &Color::Yellow,
                &Color::Black,
                &Color::Gold,
                &Color::Silver,
                &Color::Violet
            ]
        );
    }

    #[test]
    pub fn test_determine_digits() {
        let digs = Resistor::determine_digits_and_exponent(0.0).unwrap();
        assert_eq!(digs, (vec![0], 0));
        let digs = Resistor::determine_digits_and_exponent(12.0).unwrap();
        assert_eq!(digs, (vec![1, 2], 0));
        let digs = Resistor::determine_digits_and_exponent(1.2).unwrap();
        assert_eq!(digs, (vec![1, 2], -1));
        let digs = Resistor::determine_digits_and_exponent(1.0).unwrap();
        assert_eq!(digs, (vec![1, 0], -1));
        let digs = Resistor::determine_digits_and_exponent(0.12).unwrap();
        assert_eq!(digs, (vec![1, 2], -2));
        let digs = Resistor::determine_digits_and_exponent(0.01).unwrap();
        assert_eq!(digs, (vec![1, 0], -3));
        let digs = Resistor::determine_digits_and_exponent(0.123).unwrap();
        assert_eq!(digs, (vec![1, 2, 3], -3));
        let digs = Resistor::determine_digits_and_exponent(0.8).unwrap();
        assert_eq!(digs, (vec![8, 0], -2));
        let digs = Resistor::determine_digits_and_exponent(0.01003);
        assert!(digs.is_err());
    }
}
