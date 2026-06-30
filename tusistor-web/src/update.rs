use ratzilla::event::{self, KeyCode};
use tusistor_core::{
    model::{InputFocus, SelectedTab},
    update::{ColorCodesMsg, SpecsMsg, update_on_colorcodemsg, update_on_specsmsg},
};

use crate::model::Model;

pub enum Msg {
    ToggleTab,
    SpecsMsg { msg: SpecsMsg },
    ColorCodesMsg { msg: ColorCodesMsg },
}

pub fn handle_event(model: &mut Model, event: ratzilla::event::KeyEvent) {
    match (&model.selected_tab, &event.code) {
        (_, event::KeyCode::Left) if event.shift => update(model, Msg::ToggleTab),
        (_, event::KeyCode::Right) if event.shift => update(model, Msg::ToggleTab),
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
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Enter) => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::Determine,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Left) if event.ctrl => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::PrevSpecInput,
            },
        ),
        (SelectedTab::SpecsToColorCodes, event::KeyCode::Right) if event.ctrl => update(
            model,
            Msg::SpecsMsg {
                msg: SpecsMsg::NextSpecInput,
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
            let target_textarea = match model.specs_to_color.focus {
                InputFocus::Resistance => &mut model.specs_to_color.resistance_textarea,
                InputFocus::Tolerance => &mut model.specs_to_color.tolerance_textarea,
                InputFocus::Tcr => &mut model.specs_to_color.tcr_textarea,
            };

            if let Some(key) = try_convert_code(event.code) {
                let input: ratatui_textarea::Input = ratatui_textarea::Input {
                    key,
                    ctrl: event.ctrl,
                    alt: event.alt,
                    shift: event.shift,
                };
                target_textarea.input(input);
            }
        }
        _ => (),
    }
}

fn try_convert_code(code: KeyCode) -> Option<ratatui_textarea::Key> {
    match code {
        KeyCode::Char(c) => Some(ratatui_textarea::Key::Char(c)),
        KeyCode::F(n) => Some(ratatui_textarea::Key::F(n)),
        KeyCode::Backspace => Some(ratatui_textarea::Key::Backspace),
        KeyCode::Enter => Some(ratatui_textarea::Key::Enter),
        KeyCode::Left => Some(ratatui_textarea::Key::Left),
        KeyCode::Right => Some(ratatui_textarea::Key::Right),
        KeyCode::Up => Some(ratatui_textarea::Key::Up),
        KeyCode::Down => Some(ratatui_textarea::Key::Down),
        KeyCode::Tab => Some(ratatui_textarea::Key::Tab),
        KeyCode::Delete => Some(ratatui_textarea::Key::Delete),
        KeyCode::Home => Some(ratatui_textarea::Key::Home),
        KeyCode::End => Some(ratatui_textarea::Key::End),
        KeyCode::PageUp => Some(ratatui_textarea::Key::PageUp),
        KeyCode::PageDown => Some(ratatui_textarea::Key::PageDown),
        KeyCode::Esc => Some(ratatui_textarea::Key::Esc),
        KeyCode::Unidentified => None,
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
        Msg::ColorCodesMsg { msg } => update_on_colorcodemsg(&mut model.color_codes_to_specs, msg),
        Msg::SpecsMsg { msg } => update_on_specsmsg(&mut model.specs_to_color, msg),
    }
}
