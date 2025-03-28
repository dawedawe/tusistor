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
    match (&model.selected_tab, key.code) {
        (_, KeyCode::Left) | (_, KeyCode::Right) if key.modifiers == KeyModifiers::SHIFT => {
            Some(Msg::ToggleTab)
        }
        (SelectedTab::SpecsToColorCodes, KeyCode::Enter) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::Determine,
        }),
        (SelectedTab::SpecsToColorCodes, KeyCode::Tab) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::NextSpecInput,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Tab) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextBand,
        }),
        (SelectedTab::SpecsToColorCodes, KeyCode::BackTab) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::PrevSpecInput,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::BackTab) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevBand,
        }),
        (SelectedTab::SpecsToColorCodes, KeyCode::Up) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::PrevHistory,
        }),
        (SelectedTab::SpecsToColorCodes, KeyCode::Down) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::NextHistory,
        }),
        (SelectedTab::SpecsToColorCodes, KeyCode::Char('X')) => Some(Msg::SpecsMsg {
            msg: SpecsMsg::Reset,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Up) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::PrevColor,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Down) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::NextColor,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Char('3')) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::ThreeBands,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Char('4')) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FourBands,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Char('5')) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::FiveBands,
        }),
        (SelectedTab::ColorCodesToSpecs, KeyCode::Char('6')) => Some(Msg::ColorCodesMsg {
            msg: ColorCodesMsg::SixBands,
        }),
        (_, KeyCode::Esc) => Some(Msg::Exit),
        (SelectedTab::SpecsToColorCodes, _) => {
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
