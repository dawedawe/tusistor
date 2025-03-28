use crate::model::Model;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use tusistor_core::model::{InputFocus, SelectedTab};
use tusistor_core::update::{ColorCodesMsg, SpecsMsg, update_on_colorcodemsg, update_on_specsmsg};

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
        (KeyCode::Up, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::PrevHistory,
        }),
        (KeyCode::Down, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::NextHistory,
        }),
        (KeyCode::Char('X'), SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::Reset,
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
        (_, SelectedTab::SpecsToColorCodes) => {
            let target_input_state = match model.specs_to_color.focus {
                InputFocus::Resistance => &mut model.specs_to_color.resistance_input_state,
                InputFocus::Tolerance => &mut model.specs_to_color.tolerance_input_state,
                InputFocus::Tcr => &mut model.specs_to_color.tcr_input_state,
            };
            let mut input =
                Input::new(target_input_state.value.clone()).with_cursor(target_input_state.cursor);
            input.handle_event(&Event::Key(key));
            target_input_state.cursor = input.cursor();
            target_input_state.value = input.value().to_string();
            None
        }
        _ => None,
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Exit => {
            model.running = false;
        }
        Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
        Msg::ColorCodesMsg { msg } => update_on_colorcodemsg(&mut model.color_codes_to_specs, msg),
        Msg::SpecsMsg { msg } => update_on_specsmsg(&mut model.specs_to_color, msg),
    }
}

#[cfg(test)]
mod tests {
    use super::{Msg, update};
    use crate::model::Model;
    use tusistor_core::model::SelectedTab;

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
}
