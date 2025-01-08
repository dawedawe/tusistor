use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use rusistor::{self, Resistor};
use tui_input::{Input, InputRequest};

#[derive(Debug, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    input: Input,
    input_mode: InputMode,
    resistor: Option<Resistor>,
}

impl Default for App {
    fn default() -> App {
        App {
            running: true,
            input: Input::default(),
            input_mode: InputMode::Editing,
            resistor: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to start editing."),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    Span::raw("Press "),
                    Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to stop editing, "),
                    Span::raw("Press "),
                    Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to exit, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to determine the resistor."),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, chunks[0]);

        let width = chunks[0].width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .scroll((0, scroll as u16))
            .block(Block::default().borders(Borders::ALL).title("Input"));
        frame.render_widget(input, chunks[1]);
        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                frame.set_cursor_position((
                    // Put cursor past the end of the input text
                    chunks[1].x + ((self.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[1].y + 1,
                ))
            }
        }

        match &self.resistor {
            Some(resistor) => {
                let colors = resistor
                    .bands()
                    .iter()
                    .map(|c| rusistor_color_to_ratatui_color(c))
                    .collect::<Vec<Color>>();
                let chart = barchart(&colors);
                frame.render_widget(chart, chunks[2]);
            }
            None => (),
        }
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
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    self.quit();
                }
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    let input = self.input.value();
                    if input == "q" {
                        self.quit();
                    } else {
                        match input.parse::<f64>() {
                            Ok(resistance) => {
                                match Resistor::determine(resistance, Some(2.0), None) {
                                    Ok(resitor) => self.resistor = Some(resitor),
                                    Err(_) => self.resistor = None, // ToDo show error
                                }
                            }
                            _ => (),
                        }
                        self.input.reset();
                    }
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {
                    if let KeyCode::Char(c) = key.code {
                        let x: InputRequest = tui_input::InputRequest::InsertChar(c);
                        self.input.handle(x);
                    }
                }
            },
        }
    }

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
