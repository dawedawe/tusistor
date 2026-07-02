pub mod model;
pub mod update;
pub mod view;

use color_eyre::eyre::Ok;
use update::{handle_event, update};

use crate::model::TuiModel;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut tui_model = TuiModel::default();

    while tui_model.running {
        terminal.draw(|f| tusistor_core::view::view(&mut tui_model.core_model, f, view::help))?;
        if let Some(msg) = handle_event(&mut tui_model)? {
            update(&mut tui_model, msg)
        }
    }

    ratatui::restore();
    Ok(())
}
