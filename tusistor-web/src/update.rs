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
