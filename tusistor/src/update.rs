use crate::model::{InputFocus, Model, SelectedTab};
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use rusistor::{self, Resistor};
use std::str::FromStr;
use tui_input::backend::crossterm::EventHandler;
use tusistor_core::update::{ColorCodesMsg, index_to_color};

pub enum SpecsMsg {
    Determine,
    NextSpecInput,
    PrevSpecInput,
}

pub enum Msg {
    ToggleTab,
    Exit,
    SpecsMsg { msg: SpecsMsg },
    ColorCodesMsg { msg: ColorCodesMsg },
}

pub fn handle_event(model: &mut Model) -> Result<Option<Msg>> {
    match event::read()? {
        // it's important to check KeyEventKind::Press to avoid handling key release events
        Event::Key(key) if key.kind == KeyEventKind::Press => Result::Ok(on_key_event(model, key)),
        _ => Result::Ok(None),
    }
}

fn on_key_event(model: &mut Model, key: KeyEvent) -> Option<Msg> {
    match (key.code, &model.selected_tab) {
        (KeyCode::Left, _) | (KeyCode::Right, _) if key.modifiers == KeyModifiers::SHIFT => {
            Some(Msg::ToggleTab)
        }
        (KeyCode::Enter, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::Determine,
        }),
        (KeyCode::Tab, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::NextSpecInput,
        }),
        (KeyCode::Tab, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextBand,
        }),
        (KeyCode::BackTab, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::PrevSpecInput,
        }),
        (KeyCode::BackTab, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevBand,
        }),
        (KeyCode::Up, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevColor,
        }),
        (KeyCode::Down, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextColor,
        }),
        (KeyCode::Char('3'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::ThreeBands,
        }),
        (KeyCode::Char('4'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FourBands,
        }),
        (KeyCode::Char('5'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FiveBands,
        }),
        (KeyCode::Char('6'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::SixBands,
        }),
        (KeyCode::Esc, _) => Some(Msg::Exit),
        _ => {
            let target_input = match model.specs_to_color.focus {
                InputFocus::Resistance => &mut model.specs_to_color.resistance_input,
                InputFocus::Tolerance => &mut model.specs_to_color.tolerance_input,
                InputFocus::Tcr => &mut model.specs_to_color.tcr_input,
            };
            target_input.handle_event(&Event::Key(key));
            None
        }
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
        Msg::SpecsMsg {
            msg: SpecsMsg::Determine,
        } => {
            match try_determine_resistor(
                model.specs_to_color.resistance_input.value(),
                model.specs_to_color.tolerance_input.value(),
                model.specs_to_color.tcr_input.value(),
            ) {
                Ok(resistor) => {
                    model.specs_to_color.resistor = Some(resistor);
                    model.specs_to_color.error = None;
                }
                Err(e) => {
                    model.specs_to_color.resistor = None;
                    model.specs_to_color.error = Some(e);
                }
            }
            model.specs_to_color.resistance_input.reset();
            model.specs_to_color.tolerance_input.reset();
            model.specs_to_color.tcr_input.reset();
            model.specs_to_color.focus = InputFocus::Resistance;
        }
        Msg::SpecsMsg {
            msg: SpecsMsg::NextSpecInput,
        }
        | Msg::SpecsMsg {
            msg: SpecsMsg::PrevSpecInput,
        } => {
            model.specs_to_color.error = match model.specs_to_color.focus {
                InputFocus::Resistance => {
                    let value = model.specs_to_color.resistance_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        try_parse_resistance(value).err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tolerance => {
                    let value = model.specs_to_color.tolerance_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<f64>().err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tcr => {
                    let value = model.specs_to_color.tcr_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<u32>().err().map(|err| err.to_string())
                    }
                }
            };
            if model.specs_to_color.error.is_none() {
                model.specs_to_color.focus = match msg {
                    Msg::SpecsMsg {
                        msg: SpecsMsg::NextSpecInput,
                    } => model.specs_to_color.focus.next(),
                    _ => model.specs_to_color.focus.prev(),
                };
            } else {
                model.specs_to_color.resistor = None;
            }
        }
        Msg::Exit => {
            model.running = false;
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::ThreeBands,
        } => {
            model.color_codes_to_specs.resistor = Resistor::ThreeBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
            };
            model.color_codes_to_specs.selected_band =
                model.color_codes_to_specs.selected_band.min(2)
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FourBands,
        } => {
            model.color_codes_to_specs.resistor = Resistor::FourBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Brown,
            };
            model.color_codes_to_specs.selected_band =
                model.color_codes_to_specs.selected_band.min(3)
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FiveBands,
        } => {
            model.color_codes_to_specs.resistor = Resistor::FiveBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
            };
            model.color_codes_to_specs.selected_band =
                model.color_codes_to_specs.selected_band.min(4)
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::SixBands,
        } => {
            model.color_codes_to_specs.resistor = Resistor::SixBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
                band6: rusistor::Color::Black,
            };
            model.color_codes_to_specs.selected_band =
                model.color_codes_to_specs.selected_band.min(5)
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextBand,
        } => {
            model.color_codes_to_specs.selected_band = (model.color_codes_to_specs.selected_band
                + 1)
                % model.color_codes_to_specs.resistor.bands().len()
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevBand,
        } => {
            let bands_count = model.color_codes_to_specs.resistor.bands().len();
            model.color_codes_to_specs.selected_band =
                (model.color_codes_to_specs.selected_band + (bands_count - 1)) % bands_count
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextColor,
        } => {
            let current_idx: usize = *model.color_codes_to_specs.resistor.bands()
                [model.color_codes_to_specs.selected_band]
                as usize;
            let mut i: usize = 0;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i += 1;
                let next_color = index_to_color((current_idx + i) % 13);
                resistor = model
                    .color_codes_to_specs
                    .resistor
                    .with_color(next_color, model.color_codes_to_specs.selected_band);
            }
            model.color_codes_to_specs.resistor = resistor.unwrap();
        }
        Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevColor,
        } => {
            let current_idx = *model.color_codes_to_specs.resistor.bands()
                [model.color_codes_to_specs.selected_band] as usize;
            let mut i: usize = 13;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i -= 1;
                let next_color = index_to_color((current_idx + i) % 13);
                resistor = model
                    .color_codes_to_specs
                    .resistor
                    .with_color(next_color, model.color_codes_to_specs.selected_band);
            }
            model.color_codes_to_specs.resistor = resistor.unwrap();
        }
    }
}

fn try_parse_resistance(input: &str) -> Result<f64, String> {
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

fn try_determine_resistor(
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
    use super::{ColorCodesMsg, Msg, update};
    use crate::model::{Model, SelectedTab};

    #[test]
    fn test_exit_msg() {
        let mut model = Model::default();
        update(&mut model, Msg::Exit);
        assert!(!model.running)
    }

    #[test]
    fn test_toggletab_msg() {
        let mut model = Model::default();
        update(&mut model, Msg::ToggleTab);
        assert_eq!(model.selected_tab, SelectedTab::SpecsToColorCodes);
        update(&mut model, Msg::ToggleTab);
        assert_eq!(model.selected_tab, SelectedTab::ColorCodesToSpecs)
    }

    #[test]
    fn test_nbands_msg() {
        let mut model = Model::default();
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 6);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::ThreeBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 3);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FourBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 4);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FiveBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 5);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::SixBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 6);
    }
}
