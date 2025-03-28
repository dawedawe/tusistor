use std::str::FromStr;

use rusistor::{Color, Resistor};

use crate::model::{ColorCodesToSpecsModel, InputFocus, SpecsToColorModel};

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

pub enum SpecsMsg {
    Determine,
    NextSpecInput,
    PrevSpecInput,
    PrevHistory,
    NextHistory,
    Reset,
}

pub fn update_on_colorcodemsg(model: &mut ColorCodesToSpecsModel, msg: ColorCodesMsg) {
    match msg {
        ColorCodesMsg::ThreeBands => {
            model.resistor = Resistor::ThreeBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
            };
            model.selected_band = model.selected_band.min(2)
        }
        ColorCodesMsg::FourBands => {
            model.resistor = Resistor::FourBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Brown,
            };
            model.selected_band = model.selected_band.min(3)
        }
        ColorCodesMsg::FiveBands => {
            model.resistor = Resistor::FiveBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
            };
            model.selected_band = model.selected_band.min(4)
        }
        ColorCodesMsg::SixBands => {
            model.resistor = Resistor::SixBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
                band6: rusistor::Color::Black,
            };
            model.selected_band = model.selected_band.min(5)
        }
        ColorCodesMsg::NextBand => {
            model.selected_band = (model.selected_band + 1) % model.resistor.bands().len()
        }
        ColorCodesMsg::PrevBand => {
            let bands_count = model.resistor.bands().len();
            model.selected_band = (model.selected_band + (bands_count - 1)) % bands_count
        }
        ColorCodesMsg::NextColor => {
            let current_idx: usize = *model.resistor.bands()[model.selected_band] as usize;
            let mut i: usize = 0;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i += 1;
                let next_color = Color::from((current_idx + i) % 13);
                resistor = model.resistor.with_color(next_color, model.selected_band);
            }
            model.resistor = resistor.unwrap();
        }
        ColorCodesMsg::PrevColor => {
            let current_idx = *model.resistor.bands()[model.selected_band] as usize;
            let mut i: usize = 13;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i -= 1;
                let next_color = Color::from((current_idx + i) % 13);
                resistor = model.resistor.with_color(next_color, model.selected_band);
            }
            model.resistor = resistor.unwrap();
        }
    }
}
pub fn update_on_specsmsg(model: &mut SpecsToColorModel, msg: SpecsMsg) {
    match msg {
        SpecsMsg::Determine => {
            match try_determine_resistor(
                &model.resistance_input_state.value,
                &model.tolerance_input_state.value,
                &model.tcr_input_state.value,
            ) {
                Ok(resistor) => {
                    model.resistor = Some(resistor);
                    model.error = None;
                    model.history.add((
                        model.resistance_input_state.value.to_string(),
                        model.tolerance_input_state.value.to_string(),
                        model.tcr_input_state.value.to_string(),
                    ));
                    model.add_specs_to_history();
                    model.history.clear_idx();
                }
                Err(e) => {
                    model.resistor = None;
                    model.error = Some(e);
                }
            }
        }
        SpecsMsg::NextSpecInput | SpecsMsg::PrevSpecInput => {
            model.error = match model.focus {
                InputFocus::Resistance => {
                    let value = &model.resistance_input_state.value;
                    if value.trim().is_empty() {
                        None
                    } else {
                        try_parse_resistance(value).err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tolerance => {
                    let value = &model.tolerance_input_state.value;
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<f64>().err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tcr => {
                    let value = &model.tcr_input_state.value;
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<u32>().err().map(|err| err.to_string())
                    }
                }
            };
            if model.error.is_none() {
                model.focus = match msg {
                    SpecsMsg::NextSpecInput => model.focus.next(),
                    _ => model.focus.prev(),
                };
            } else {
                model.resistor = None;
            }
        }
        SpecsMsg::PrevHistory => {
            model.history.prev();
            model.set_specs_from_history();
        }
        SpecsMsg::NextHistory => {
            model.history.next();
            model.set_specs_from_history();
        }
        SpecsMsg::Reset => *model = SpecsToColorModel::default(),
    }
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

#[cfg(test)]
mod tests {
    use super::ColorCodesMsg;
    use crate::{
        model::{ColorCodesToSpecsModel, InputState, SpecsToColorModel},
        update::{SpecsMsg, update_on_colorcodemsg, update_on_specsmsg},
    };

    #[test]
    fn test_nbands_msg() {
        let mut model = ColorCodesToSpecsModel::default();
        assert_eq!(model.resistor.bands().len(), 6);

        update_on_colorcodemsg(&mut model, ColorCodesMsg::ThreeBands);
        assert_eq!(model.resistor.bands().len(), 3);

        update_on_colorcodemsg(&mut model, ColorCodesMsg::FourBands);
        assert_eq!(model.resistor.bands().len(), 4);

        update_on_colorcodemsg(&mut model, ColorCodesMsg::FiveBands);
        assert_eq!(model.resistor.bands().len(), 5);

        update_on_colorcodemsg(&mut model, ColorCodesMsg::SixBands);
        assert_eq!(model.resistor.bands().len(), 6);
    }

    #[test]
    fn test_reset_msg() {
        let mut model = SpecsToColorModel {
            resistance_input_state: InputState {
                value: String::from("z"),
                cursor: 1,
            },
            tolerance_input_state: InputState {
                value: String::from("z"),
                cursor: 1,
            },
            tcr_input_state: InputState {
                value: String::from("z"),
                cursor: 1,
            },
            ..Default::default()
        };
        update_on_specsmsg(&mut model, SpecsMsg::Reset);
        assert_eq!(model.resistance_input_state.value, "");
        assert_eq!(model.tolerance_input_state.value, "");
        assert_eq!(model.tcr_input_state.value, "");
    }

    #[test]
    fn test_history() {
        let mut model = SpecsToColorModel {
            resistance_input_state: InputState {
                value: String::from("1"),
                cursor: 1,
            },
            tolerance_input_state: InputState {
                value: String::from("2"),
                cursor: 1,
            },
            tcr_input_state: InputState {
                value: String::from("5"),
                cursor: 1,
            },
            ..Default::default()
        };
        update_on_specsmsg(&mut model, SpecsMsg::Determine);

        model.resistance_input_state = InputState {
            value: String::from("2"),
            cursor: 1,
        };
        model.tolerance_input_state = InputState {
            value: String::from("5"),
            cursor: 1,
        };

        model.tcr_input_state = InputState {
            value: String::from("1"),
            cursor: 1,
        };
        update_on_specsmsg(&mut model, SpecsMsg::Determine);
        update_on_specsmsg(&mut model, SpecsMsg::PrevHistory);
        update_on_specsmsg(&mut model, SpecsMsg::PrevHistory);
        assert_eq!(model.resistance_input_state.value, "1");
        assert_eq!(model.tolerance_input_state.value, "2");
        assert_eq!(model.tcr_input_state.value, "5");
    }
}
