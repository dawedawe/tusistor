use std::str::FromStr;

use rusistor::Resistor;

pub enum ColorCodesMsg {
    ThreeBands,
    FourBands,
    FiveBands,
    SixBands,
    NextBand,
    PrevBand,
    NextColor,
    PrevColor,
}

pub fn try_parse_resistance(input: &str) -> Result<f64, String> {
    match input.parse::<f64>() {
        Ok(t) => Ok(t),
        Err(e) => match engineering_repr::EngineeringQuantity::<i64>::from_str(input) {
            Ok(t) => {
                let r: i64 = t.into();
                let r = r as f64;
                Ok(r)
            }
            Err(_) => Err(format!("invalid input for resistance: {}", e)),
        },
    }
}

pub fn try_determine_resistor(
    resistance_input: &str,
    tolerance_input: &str,
    tcr_input: &str,
) -> Result<Resistor, String> {
    let resistance = try_parse_resistance(resistance_input);
    let tolerance = if tolerance_input.is_empty() {
        Ok(None)
    } else {
        match tolerance_input.parse::<f64>() {
            Ok(t) => Ok(Some(t)),
            Err(e) => Err(format!("invalid input for tolerance: {}", e)),
        }
    };

    let tcr = if tcr_input.is_empty() {
        Ok(None)
    } else {
        match tcr_input.parse::<u32>() {
            Ok(t) => Ok(Some(t)),
            Err(e) => Err(format!("invalid input for tcr: {}", e)),
        }
    };

    match (resistance, tolerance, tcr) {
        (Ok(resistance), Ok(tolerance), Ok(tcr)) => {
            match Resistor::determine(resistance, tolerance, tcr) {
                Ok(resistor) => Ok(resistor),
                Err(e) => Err(format!(
                    "could not determine a resistor for these inputs: {}",
                    e
                )),
            }
        }
        (res, tol, tcr) => {
            let mut error_msg: String = String::from("");
            if let Err(res_error) = res {
                error_msg.push_str(res_error.to_string().as_str());
            }
            if let Err(tol_error) = tol {
                error_msg.push('\n');
                error_msg.push_str(tol_error.to_string().as_str());
            }
            if let Err(tcr_error) = tcr {
                error_msg.push('\n');
                error_msg.push_str(tcr_error.to_string().as_str());
            }
            if error_msg.is_empty() {
                panic!("unknown error");
            } else {
                Err(error_msg)
            }
        }
    }
}
