pub mod app;

use app::{handle_event, update, view, Model};
use color_eyre::eyre::Ok;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut model = Model::default();

    while model.running {
        terminal.draw(|f| view(&model, f))?;
        if let Some(msg) = handle_event(&mut model)? {
            update(&mut model, msg)
        }
    }

    ratatui::restore();
    Ok(())
}
