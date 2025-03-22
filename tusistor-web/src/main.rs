pub mod input;
pub mod model;
pub mod update;
pub mod view;

use model::Model;
use ratzilla::{DomBackend, WebRenderer};
use std::{cell::RefCell, io, rc::Rc};
use update::handle_event;
use view::view;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = ratzilla::ratatui::Terminal::new(backend)?;
    let model = Rc::new(RefCell::new(Model::default()));

    terminal.on_key_event({
        let model = model.clone();
        move |key_event| handle_event(&mut model.borrow_mut(), key_event)
    });

    terminal.draw_web(move |frame| {
        view(&model.borrow(), frame);
    });

    Ok(())
}
