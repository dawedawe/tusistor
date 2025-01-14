use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
    DefaultTerminal, Frame,
};
use rusistor::{self, Resistor};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Debug)]
pub struct App {
    running: bool,
    resistance_input: Input,
    tolerance_input: Input,
    tcr_input: Input,
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

        let main_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(chunks[2]);

        let help_msg_rect = chunks[0];
        let resistance_rect = input_rects[0];
        let tolerance_rect = input_rects[1];
        let tcr_rect = input_rects[2];
        let main_rect = main_rects[1];

        let (msg, style) = (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to move to the next input, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to determine the resistor."),
            ],
            Style::default(),
        );
        let text = Text::from(Line::from(msg)).style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_msg_rect);

        let resistance_width = resistance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let resistance_scroll = self
            .resistance_input
            .visual_scroll(resistance_width as usize);
        let resistance_paragraph = Paragraph::new(self.resistance_input.value())
            .style(Style::default().fg(Color::Yellow))
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
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, tolerance_scroll as u16))
            .block(Block::default().borders(Borders::ALL).title(" Tolerance "));
        frame.render_widget(tolerance_paragraph, tolerance_rect);

        let tcr_width = tcr_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
        let tcr_scroll = self.tcr_input.visual_scroll(tcr_width as usize);
        let tcr_paragraph = Paragraph::new(self.tcr_input.value())
            .style(Style::default().fg(Color::Yellow))
            .scroll((0, tcr_scroll as u16))
            .block(Block::default().borders(Borders::ALL).title(" TCR "));
        frame.render_widget(tcr_paragraph, tcr_rect);

        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        let (rect, input, scroll) = match self.focus {
            0 => (resistance_rect, &self.resistance_input, resistance_scroll),
            1 => (tolerance_rect, &self.tolerance_input, tolerance_scroll),
            _ => (tcr_rect, &self.tcr_input, tcr_scroll),
        };
        frame.set_cursor_position((
            // Put cursor past the end of the input text
            rect.x + ((input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            // Move one line down, from the border to the input line
            rect.y + 1,
        ));

        match &self.resistor {
            Some(resistor) => {
                let colors = resistor
                    .bands()
                    .iter()
                    .map(|c| rusistor_color_to_ratatui_color(c))
                    .collect::<Vec<(Color, String)>>();
                let specs = resistor.specs();
                let chart = barchart(&colors, specs.ohm, specs.tolerance, specs.tcr);
                frame.render_widget(chart, main_rect);
            }
            None => {
                if let Some(e) = &self.error {
                    let text = Text::from(e.to_string());
                    let error_message = Paragraph::new(text);
                    frame.render_widget(error_message, main_rect);
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
        match key.code {
            KeyCode::Enter => {
                let resistance_input_value = self.resistance_input.value();
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
            KeyCode::Tab => self.focus = (self.focus + 1) % 3,
            KeyCode::Esc => self.quit(),
            _ => {
                let target_input = match self.focus {
                    0 => &mut self.resistance_input,
                    1 => &mut self.tolerance_input,
                    _ => &mut self.tcr_input,
                };
                target_input.handle_event(&Event::Key(key));
            }
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

fn barchart(bands: &[(Color, String)], ohm: f64, tolerance: f64, tcr: Option<u32>) -> BarChart {
    let bars: Vec<Bar> = bands.iter().map(|color| bar(color)).collect();
    let mut s = format!("Resistance: {}Ω - Tolerance: ±{}%", ohm, tolerance * 100.0);
    if let Some(tcr) = tcr {
        s.push_str(format!(" - TCR: {}(ppm/K)", tcr).as_str());
    }
    let title = Line::from(s).centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(Block::new().title(title).borders(Borders::all()))
        .bar_width(10)
}

fn bar((color, name): &(Color, String)) -> Bar {
    Bar::default()
        .value(100)
        .text_value("".to_string())
        .label(Line::from(name.as_str()))
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
