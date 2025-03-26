use ratzilla::event;
use tusistor_core::{
    model::{InputFocus, SelectedTab},
    update::{ColorCodesMsg, SpecsMsg, update_on_colorcodemsg, update_on_specsmsg},
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
        Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
        Msg::ColorCodesMsg { msg } => update_on_colorcodemsg(&mut model.color_codes_to_specs, msg),
        Msg::SpecsMsg { msg } => update_on_specsmsg(&mut model.specs_to_color, msg),
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
