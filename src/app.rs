use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Bar, BarChart, BarGroup, Block, Borders, Paragraph},
    Frame,
};
use rusistor::{self, Resistor};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Debug, Default)]
pub enum InputFocus {
    #[default]
    Resistance,
    Tolerance,
    Tcr,
}

impl InputFocus {
    fn next(&self) -> InputFocus {
        match self {
            InputFocus::Resistance => InputFocus::Tolerance,
            InputFocus::Tolerance => InputFocus::Tcr,
            InputFocus::Tcr => InputFocus::Resistance,
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub running: bool,
    pub resistance_input: Input,
    pub tolerance_input: Input,
    pub tcr_input: Input,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub error: Option<String>,
}

impl Default for Model {
    fn default() -> Model {
        Model {
            running: true,
            resistance_input: Input::default(),
            tolerance_input: Input::default(),
            tcr_input: Input::default(),
            focus: InputFocus::default(),
            resistor: None,
            error: None,
        }
    }
}

pub fn view(model: &Model, frame: &mut Frame) {
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
    let resistance_scroll = model
        .resistance_input
        .visual_scroll(resistance_width as usize);
    let resistance_paragraph = Paragraph::new(model.resistance_input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, resistance_scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Resistance (Ohm) "),
        );
    frame.render_widget(resistance_paragraph, resistance_rect);

    let tolerance_width = tolerance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let tolerance_scroll = model
        .tolerance_input
        .visual_scroll(tolerance_width as usize);
    let tolerance_paragraph = Paragraph::new(model.tolerance_input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, tolerance_scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Tolerance (%) "),
        );
    frame.render_widget(tolerance_paragraph, tolerance_rect);

    let tcr_width = tcr_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let tcr_scroll = model.tcr_input.visual_scroll(tcr_width as usize);
    let tcr_paragraph = Paragraph::new(model.tcr_input.value())
        .style(Style::default().fg(Color::Yellow))
        .scroll((0, tcr_scroll as u16))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" TCR (ppm/K) "),
        );
    frame.render_widget(tcr_paragraph, tcr_rect);

    // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
    let (rect, input, scroll) = match model.focus {
        InputFocus::Resistance => (resistance_rect, &model.resistance_input, resistance_scroll),
        InputFocus::Tolerance => (tolerance_rect, &model.tolerance_input, tolerance_scroll),
        InputFocus::Tcr => (tcr_rect, &model.tcr_input, tcr_scroll),
    };
    frame.set_cursor_position((
        // Put cursor past the end of the input text
        rect.x + ((input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        rect.y + 1,
    ));

    if let Some(resistor) = &model.resistor {
        let colors = resistor
            .bands()
            .iter()
            .map(|c| rusistor_color_to_ratatui_color(c))
            .collect::<Vec<(Color, String)>>();
        let specs = resistor.specs();
        let chart = barchart(&colors, specs.ohm, specs.tolerance, specs.tcr);
        frame.render_widget(chart, main_rect);
    }
    if let Some(e) = &model.error {
        let text = Text::from(e.to_string());
        let error_message = Paragraph::new(text).style(Style::default().fg(Color::Red));
        frame.render_widget(error_message, main_rect);
    }
}

pub enum Msg {
    Determine,
    FocusNext,
    Exit,
}

pub fn handle_event(model: &mut Model) -> Result<Option<Msg>> {
    match event::read()? {
        // it's important to check KeyEventKind::Press to avoid handling key release events
        Event::Key(key) if key.kind == KeyEventKind::Press => Result::Ok(on_key_event(model, key)),
        _ => Result::Ok(None),
    }
}

fn on_key_event(model: &mut Model, key: KeyEvent) -> Option<Msg> {
    match key.code {
        KeyCode::Enter => Some(Msg::Determine),
        KeyCode::Tab => Some(Msg::FocusNext),
        KeyCode::Esc => Some(Msg::Exit),
        _ => {
            let target_input = match model.focus {
                InputFocus::Resistance => &mut model.resistance_input,
                InputFocus::Tolerance => &mut model.tolerance_input,
                InputFocus::Tcr => &mut model.tcr_input,
            };
            target_input.handle_event(&Event::Key(key));
            None
        }
    }
}

pub fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Determine => {
            match try_determine_resistor(
                model.resistance_input.value(),
                model.tolerance_input.value(),
                model.tcr_input.value(),
            ) {
                Ok(resistor) => {
                    model.resistor = Some(resistor);
                    model.error = None;
                }
                Err(e) => {
                    model.resistor = None;
                    model.error = Some(e);
                }
            }
            model.resistance_input.reset();
            model.tolerance_input.reset();
            model.tcr_input.reset();
            model.focus = InputFocus::Resistance;
        }
        Msg::FocusNext => {
            model.error = match model.focus {
                InputFocus::Resistance => {
                    let value = model.resistance_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<f64>().err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tolerance => {
                    let value = model.tolerance_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<f64>().err().map(|err| err.to_string())
                    }
                }
                InputFocus::Tcr => {
                    let value = model.tcr_input.value();
                    if value.trim().is_empty() {
                        None
                    } else {
                        value.parse::<u32>().err().map(|err| err.to_string())
                    }
                }
            };
            if model.error.is_none() {
                model.focus = model.focus.next();
            } else {
                model.resistor = None;
            }
        }
        Msg::Exit => {
            model.running = false;
        }
    }
}

fn try_determine_resistor(
    resistance_input: &str,
    tolerance_input: &str,
    tcr_input: &str,
) -> Result<Resistor, String> {
    let resistance = match resistance_input.parse::<f64>() {
        Ok(t) => Ok(t),
        Err(e) => Err(format!("invalid input for resistance: {}", e)),
    };

    let tolerance = if tolerance_input.is_empty() {
        Ok(None)
    } else {
        match tolerance_input.parse::<f64>() {
            Ok(t) => Ok(Some(t)),
            Err(e) => Err(format!("invalid input for tolerance: {}", e)),
        }
    };

    let tcr = if tcr_input.is_empty() {
        Ok(None)
    } else {
        match tcr_input.parse::<u32>() {
            Ok(t) => Ok(Some(t)),
            Err(e) => Err(format!("invalid input for tcr: {}", e)),
        }
    };

    match (resistance, tolerance, tcr) {
        (Ok(resistance), Ok(tolerance), Ok(tcr)) => {
            match Resistor::determine(resistance, tolerance, tcr) {
                Ok(resistor) => Ok(resistor),
                Err(e) => Err(format!(
                    "could not determine a resistor for these inputs: {}",
                    e
                )),
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
                panic!("unknown error");
            } else {
                Err(error_msg)
            }
        }
    }
}

fn barchart(bands: &[(Color, String)], ohm: f64, tolerance: f64, tcr: Option<u32>) -> BarChart {
    let bars: Vec<Bar> = bands.iter().map(|color| bar(color)).collect();
    let tcr = if let Some(tcr) = tcr {
        format!(" - TCR: {}(ppm/K)", tcr)
    } else {
        String::from("")
    };
    let title = format!(
        "Resistance: {}Ω - Tolerance: ±{}%{}",
        ohm,
        tolerance * 100.0,
        tcr
    );
    let title = Line::from(title).centered();
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
