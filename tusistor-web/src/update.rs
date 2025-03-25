use ratzilla::event;
use rusistor::{self, Color, Resistor};
use tusistor_core::{
    model::{InputFocus, SelectedTab, SpecsToColorModel},
    update::{ColorCodesMsg, SpecsMsg, try_determine_resistor, try_parse_resistance},
};

use crate::{input::WebInput, model::Model};

pub enum Msg {
    ToggleTab,
    SpecsMsg { msg: SpecsMsg },
    ColorCodesMsg { msg: ColorCodesMsg },
}

pub fn handle_event(model: &mut Model, event: ratzilla::event::KeyEvent) {
    match (&model.selected_tab, &event.code) {
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Char('3')) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::ThreeBands,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Char('4')) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FourBands,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Char('5')) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FiveBands,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Char('6')) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::SixBands,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Up) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevColor,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Down) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextColor,
            },
        ),
        (_, event::KeyCode::Left) if event.shift => update(model, Msg::ToggleTab),
        (_, event::KeyCode::Right) if event.shift => update(model, Msg::ToggleTab),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Left) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevBand,
            },
        ),
        (SelectedTab::ColorCodesToSpecs, event::KeyCode::Right) => update(
            model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextBand,
            },
        ),

        (SelectedTab::SpecsToColorCodes, event::KeyCode::Enter) => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::Determine,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Right) if event.ctrl => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::NextSpecInput,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Left) if event.ctrl => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::PrevSpecInput,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Up) => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::PrevHistory,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Down) => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::NextHistory,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Char('X')) => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::Reset,
            },
        ),
        (SelectedTab::SpecsToColorCodes, _) => {
            let target_input_state = match model.specs_to_color.focus {
                InputFocus::Resistance => &mut model.specs_to_color.resistance_input_state,
                InputFocus::Tolerance => &mut model.specs_to_color.tolerance_input_state,
                InputFocus::Tcr => &mut model.specs_to_color.tcr_input_state,
            };
            let mut input = WebInput::new(target_input_state.value.clone())
                .with_cursor(target_input_state.cursor);
            input.handle_event(&event);
            target_input_state.cursor = input.cursor();
            target_input_state.value = input.value().to_string();
        }
        _ => (),
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::ColorCodesMsg { msg } => match msg {
            ColorCodesMsg::ThreeBands => {
                model.color_codes_to_specs.resistor = Resistor::ThreeBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(2)
            }
            ColorCodesMsg::FourBands => {
                model.color_codes_to_specs.resistor = Resistor::FourBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                    band4: rusistor::Color::Brown,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(3)
            }
            ColorCodesMsg::FiveBands => {
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
            ColorCodesMsg::SixBands => {
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
            ColorCodesMsg::NextBand => {
                model.color_codes_to_specs.selected_band =
                    (model.color_codes_to_specs.selected_band + 1)
                        % model.color_codes_to_specs.resistor.bands().len()
            }
            ColorCodesMsg::PrevBand => {
                let bands_count = model.color_codes_to_specs.resistor.bands().len();
                model.color_codes_to_specs.selected_band =
                    (model.color_codes_to_specs.selected_band + (bands_count - 1)) % bands_count
            }
            ColorCodesMsg::NextColor => {
                let current_idx: usize = *model.color_codes_to_specs.resistor.bands()
                    [model.color_codes_to_specs.selected_band]
                    as usize;
                let mut i: usize = 0;
                let mut resistor = Err("".to_string());
                while resistor.is_err() {
                    i += 1;
                    let next_color = Color::from((current_idx + i) % 13);
                    resistor = model
                        .color_codes_to_specs
                        .resistor
                        .with_color(next_color, model.color_codes_to_specs.selected_band);
                }
                model.color_codes_to_specs.resistor = resistor.unwrap();
            }
            ColorCodesMsg::PrevColor => {
                let current_idx = *model.color_codes_to_specs.resistor.bands()
                    [model.color_codes_to_specs.selected_band]
                    as usize;
                let mut i: usize = 13;
                let mut resistor = Err("".to_string());
                while resistor.is_err() {
                    i -= 1;
                    let next_color = Color::from((current_idx + i) % 13);
                    resistor = model
                        .color_codes_to_specs
                        .resistor
                        .with_color(next_color, model.color_codes_to_specs.selected_band);
                }
                model.color_codes_to_specs.resistor = resistor.unwrap();
            }
        },
        Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
        Msg::SpecsMsg { msg } => match msg {
            SpecsMsg::Determine => {
                match try_determine_resistor(
                    &model.specs_to_color.resistance_input_state.value,
                    &model.specs_to_color.tolerance_input_state.value,
                    &model.specs_to_color.tcr_input_state.value,
                ) {
                    Ok(resistor) => {
                        model.specs_to_color.resistor = Some(resistor);
                        model.specs_to_color.error = None;
                        model.specs_to_color.history.add((
                            model
                                .specs_to_color
                                .resistance_input_state
                                .value
                                .to_string(),
                            model.specs_to_color.tolerance_input_state.value.to_string(),
                            model.specs_to_color.tcr_input_state.value.to_string(),
                        ));
                        model.specs_to_color.add_specs_to_history();
                        model.specs_to_color.history.clear_idx();
                    }
                    Err(e) => {
                        model.specs_to_color.resistor = None;
                        model.specs_to_color.error = Some(e);
                    }
                }
            }
            SpecsMsg::NextSpecInput | SpecsMsg::PrevSpecInput => {
                model.specs_to_color.error = match model.specs_to_color.focus {
                    InputFocus::Resistance => {
                        let value = &model.specs_to_color.resistance_input_state.value;
                        if value.trim().is_empty() {
                            None
                        } else {
                            try_parse_resistance(value).err().map(|err| err.to_string())
                        }
                    }
                    InputFocus::Tolerance => {
                        let value = &model.specs_to_color.tolerance_input_state.value;
                        if value.trim().is_empty() {
                            None
                        } else {
                            value.parse::<f64>().err().map(|err| err.to_string())
                        }
                    }
                    InputFocus::Tcr => {
                        let value = &model.specs_to_color.tcr_input_state.value;
                        if value.trim().is_empty() {
                            None
                        } else {
                            value.parse::<u32>().err().map(|err| err.to_string())
                        }
                    }
                };
                if model.specs_to_color.error.is_none() {
                    model.specs_to_color.focus = match msg {
                        SpecsMsg::NextSpecInput => model.specs_to_color.focus.next(),
                        _ => model.specs_to_color.focus.prev(),
                    };
                } else {
                    model.specs_to_color.resistor = None;
                }
            }
            SpecsMsg::PrevHistory => {
                model.specs_to_color.history.prev();
                model.specs_to_color.set_specs_from_history();
            }
            SpecsMsg::NextHistory => {
                model.specs_to_color.history.next();
                model.specs_to_color.set_specs_from_history();
            }
            SpecsMsg::Reset => model.specs_to_color = SpecsToColorModel::default(),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::Model, update};

    use super::{Msg, update};
    use tusistor_core::{model::InputState, update::ColorCodesMsg};

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

    #[test]
    fn test_reset_msg() {
        let mut model = Model::default();
        model.specs_to_color.resistance_input_state = InputState {
            value: String::from("z"),
            cursor: 1,
        };
        model.specs_to_color.tolerance_input_state = InputState {
            value: String::from("z"),
            cursor: 1,
        };
        model.specs_to_color.tcr_input_state = InputState {
            value: String::from("z"),
            cursor: 1,
        };

        update(
            &mut model,
            Msg::SpecsMsg {
                msg: tusistor_core::update::SpecsMsg::Reset,
            },
        );
        assert_eq!(model.specs_to_color.resistance_input_state.value, "");
        assert_eq!(model.specs_to_color.tolerance_input_state.value, "");
        assert_eq!(model.specs_to_color.tcr_input_state.value, "");
    }

    #[test]
    fn test_history() {
        let mut model = Model::default();
        model.specs_to_color.resistance_input_state = InputState {
            value: String::from("1"),
            cursor: 1,
        };
        model.specs_to_color.tolerance_input_state = InputState {
            value: String::from("2"),
            cursor: 1,
        };

        model.specs_to_color.tcr_input_state = InputState {
            value: String::from("5"),
            cursor: 1,
        };
        update(
            &mut model,
            Msg::SpecsMsg {
                msg: update::SpecsMsg::Determine,
            },
        );

        model.specs_to_color.resistance_input_state = InputState {
            value: String::from("2"),
            cursor: 1,
        };
        model.specs_to_color.tolerance_input_state = InputState {
            value: String::from("5"),
            cursor: 1,
        };

        model.specs_to_color.tcr_input_state = InputState {
            value: String::from("1"),
            cursor: 1,
        };
        update(
            &mut model,
            Msg::SpecsMsg {
                msg: update::SpecsMsg::Determine,
            },
        );
        update(
            &mut model,
            Msg::SpecsMsg {
                msg: update::SpecsMsg::PrevHistory,
            },
        );
        update(
            &mut model,
            Msg::SpecsMsg {
                msg: update::SpecsMsg::PrevHistory,
            },
        );
        assert_eq!(model.specs_to_color.resistance_input_state.value, "1");
        assert_eq!(model.specs_to_color.tolerance_input_state.value, "2");
        assert_eq!(model.specs_to_color.tcr_input_state.value, "5");
    }
}
