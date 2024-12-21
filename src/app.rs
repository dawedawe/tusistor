use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
    DefaultTerminal, Frame,
};
use rusistor::{self, Resistor};

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        let resitor = Resistor::determine(654.0, Some(10.0), Some(5)).unwrap();
        let colors = resitor
            .bands()
            .iter()
            .map(|c| rusistor_color_to_ratatui_color(c))
            .collect::<Vec<Color>>();
        let chart = barchart(&colors);
        frame.render_widget(chart, frame.area())
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

fn barchart(bands: &[Color]) -> BarChart {
    let bars: Vec<Bar> = bands.iter().map(|color| bar(color)).collect();
    let title = Line::from("TUsIstor").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(5)
        .bg(Color::Rgb(153, 204, 255))
}

fn bar(color: &Color) -> Bar {
    Bar::default()
        .value(100)
        .text_value(String::new())
        .style(bar_style(color))
}

fn bar_style(color: &Color) -> Style {
    Style::new().fg(*color)
}

fn rusistor_color_to_ratatui_color(color: &rusistor::Color) -> Color {
    match color {
        rusistor::Color::Black => Color::Black,
        rusistor::Color::Brown => Color::Rgb(165, 42, 42),
        rusistor::Color::Red => Color::Red,
        rusistor::Color::Orange => Color::Rgb(255, 165, 0),
        rusistor::Color::Yellow => Color::Yellow,
        rusistor::Color::Green => Color::Green,
        rusistor::Color::Blue => Color::Blue,
        rusistor::Color::Violet => Color::Rgb(148, 0, 211),
        rusistor::Color::Grey => Color::Gray,
        rusistor::Color::White => Color::White,
        rusistor::Color::Gold => Color::Rgb(255, 215, 0),
        rusistor::Color::Silver => Color::Rgb(192, 192, 192),
        rusistor::Color::Pink => Color::Rgb(255, 105, 180),
    }
}
