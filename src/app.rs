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
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Debug, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct App {
    running: bool,
    resistance_input: Input,
    tolerance_input: Input,
    tcr_input: Input,
    input_mode: InputMode,
    focus: usize,
    resistor: Option<Resistor>,
    error: Option<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            running: true,
            resistance_input: Input::default(),
            tolerance_input: Input::default(),
            tcr_input: Input::default(),
            input_mode: InputMode::Editing,
            focus: 0,
            resistor: None,
            error: None,
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
        let input_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(chunks[1]);
        let resistance_rect = input_rects[0];
        let tolerance_rect = input_rects[1];
        let tcr_rect = input_rects[2];

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

        let resistance_width = resistance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let resistance_scroll = self
            .resistance_input
            .visual_scroll(resistance_width as usize);
        let resistance_paragraph = Paragraph::new(self.resistance_input.value())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .scroll((0, resistance_scroll as u16))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Resistance (Ohm) "),
            );
        frame.render_widget(resistance_paragraph, resistance_rect);

        let tolerance_width = tolerance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let tolerance_scroll = self.tolerance_input.visual_scroll(tolerance_width as usize);
        let tolerance_paragraph = Paragraph::new(self.tolerance_input.value())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .scroll((0, tolerance_scroll as u16))
            .block(Block::default().borders(Borders::ALL).title(" Tolerance "));
        frame.render_widget(tolerance_paragraph, tolerance_rect);

        let tcr_width = tcr_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let tcr_scroll = self.tcr_input.visual_scroll(tcr_width as usize);
        let tcr_paragraph = Paragraph::new(self.tcr_input.value())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .scroll((0, tcr_scroll as u16))
            .block(Block::default().borders(Borders::ALL).title(" TCR "));
        frame.render_widget(tcr_paragraph, tcr_rect);

        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering

                match self.focus {
                    0 => frame.set_cursor_position((
                        // Put cursor past the end of the input text
                        resistance_rect.x
                            + ((self.resistance_input.visual_cursor()).max(resistance_scroll)
                                - resistance_scroll) as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        resistance_rect.y + 1,
                    )),
                    1 => frame.set_cursor_position((
                        // Put cursor past the end of the input text
                        tolerance_rect.x
                            + ((self.tolerance_input.visual_cursor()).max(tolerance_scroll)
                                - tolerance_scroll) as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        tolerance_rect.y + 1,
                    )),
                    _ => frame.set_cursor_position((
                        // Put cursor past the end of the input text
                        tcr_rect.x
                            + ((self.tcr_input.visual_cursor()).max(tcr_scroll) - tcr_scroll)
                                as u16
                            + 1,
                        // Move one line down, from the border to the input line
                        tcr_rect.y + 1,
                    )),
                }
            }
        }

        match &self.resistor {
            Some(resistor) => {
                let colors = resistor
                    .bands()
                    .iter()
                    .map(|c| rusistor_color_to_ratatui_color(c))
                    .collect::<Vec<(Color, String)>>();
                let chart = barchart(&colors);
                frame.render_widget(chart, chunks[2]);
            }
            None => {
                if let Some(e) = &self.error {
                    let text = Text::from(e.to_string());
                    let error_message = Paragraph::new(text);
                    frame.render_widget(error_message, chunks[2]);
                }
            }
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
                    let resistance_input_value = self.resistance_input.value();
                    if resistance_input_value == "q" {
                        self.quit();
                    } else {
                        self.resistor = None;

                        let resistance = match resistance_input_value.parse::<f64>() {
                            Ok(t) => Ok(t),
                            Err(e) => Err(format!("invalid input for resistance: {}", e)),
                        };

                        let tolerance_input_value = self.tolerance_input.value();
                        let tolerance = if tolerance_input_value.is_empty() {
                            Ok(None)
                        } else {
                            match tolerance_input_value.parse::<f64>() {
                                Ok(t) => Ok(Some(t)),
                                Err(e) => Err(format!("invalid input for tolerance: {}", e)),
                            }
                        };

                        let tcr_input_value = self.tcr_input.value();
                        let tcr = if tcr_input_value.is_empty() {
                            Ok(None)
                        } else {
                            match tcr_input_value.parse::<u32>() {
                                Ok(t) => Ok(Some(t)),
                                Err(e) => Err(format!("invalid input for tcr: {}", e)),
                            }
                        };

                        match (resistance, tolerance, tcr) {
                            (Ok(resistance), Ok(tolerance), Ok(tcr)) => {
                                match Resistor::determine(resistance, tolerance, tcr) {
                                    Ok(resitor) => {
                                        self.error = None;
                                        self.resistor = Some(resitor)
                                    }
                                    Err(e) => {
                                        self.error = Some(format!(
                                            "could not determine a resistor for these inputs: {}",
                                            e
                                        ));
                                    }
                                }
                            }
                            (res, tol, tcr) => {
                                let mut error_msg: String = String::from("");
                                if let Err(res_error) = res {
                                    error_msg.push_str(res_error.to_string().as_str());
                                }
                                if let Err(tol_error) = tol {
                                    error_msg.push('\n');
                                    error_msg.push_str(tol_error.to_string().as_str());
                                }
                                if let Err(tcr_error) = tcr {
                                    error_msg.push('\n');
                                    error_msg.push_str(tcr_error.to_string().as_str());
                                }
                                if error_msg.is_empty() {
                                    self.error = None
                                } else {
                                    self.error = Some(error_msg)
                                }
                            }
                        }

                        self.reset_inputs();
                    }
                }
                KeyCode::Tab => self.focus = (self.focus + 1) % 3,
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {
                    let target_input = match self.focus {
                        0 => &mut self.resistance_input,
                        1 => &mut self.tolerance_input,
                        _ => &mut self.tcr_input,
                    };
                    target_input.handle_event(&Event::Key(key));
                }
            },
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn reset_inputs(&mut self) {
        self.resistance_input.reset();
        self.tolerance_input.reset();
        self.tcr_input.reset();
        self.focus = 0;
    }
}

fn barchart(bands: &[(Color, String)]) -> BarChart {
    let bars: Vec<Bar> = bands.iter().map(|color| bar(color)).collect();
    let title = Line::from("TUsIstor").centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title))
        .bar_width(10)
        .bg(Color::Rgb(153, 204, 255))
}

fn bar((color, name): &(Color, String)) -> Bar {
    Bar::default()
        .value(100)
        .text_value(name.to_string())
        .style(bar_style(color))
}

fn bar_style(color: &Color) -> Style {
    Style::new().fg(*color)
}

fn rusistor_color_to_ratatui_color(color: &rusistor::Color) -> (Color, String) {
    match color {
        rusistor::Color::Black => (Color::Black, String::from("black")),
        rusistor::Color::Brown => (Color::Rgb(165, 42, 42), String::from("brown")),
        rusistor::Color::Red => (Color::Red, String::from("red")),
        rusistor::Color::Orange => (Color::Rgb(255, 165, 0), String::from("organge")),
        rusistor::Color::Yellow => (Color::Yellow, String::from("yellow")),
        rusistor::Color::Green => (Color::Green, String::from("green")),
        rusistor::Color::Blue => (Color::Blue, String::from("blue")),
        rusistor::Color::Violet => (Color::Rgb(148, 0, 211), String::from("violet")),
        rusistor::Color::Grey => (Color::Gray, String::from("grey")),
        rusistor::Color::White => (Color::White, String::from("white")),
        rusistor::Color::Gold => (Color::Rgb(255, 215, 0), String::from("gold")),
        rusistor::Color::Silver => (Color::Rgb(192, 192, 192), String::from("silver")),
        rusistor::Color::Pink => (Color::Rgb(255, 105, 180), String::from("pink")),
    }
}
