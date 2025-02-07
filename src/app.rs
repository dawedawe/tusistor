use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, List, ListDirection, ListItem, ListState,
        Paragraph, Tabs,
    },
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

    fn prev(&self) -> InputFocus {
        match self {
            InputFocus::Resistance => InputFocus::Tcr,
            InputFocus::Tolerance => InputFocus::Resistance,
            InputFocus::Tcr => InputFocus::Tolerance,
        }
    }
}

#[derive(Debug)]
pub struct Model {
    pub running: bool,
    pub selected_tab_index: usize,
    pub resistance_input: Input,
    pub tolerance_input: Input,
    pub tcr_input: Input,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub error: Option<String>,
    pub selected_band: usize,
    pub selected_in_bands: [usize; 6],
}

impl Default for Model {
    fn default() -> Model {
        Model {
            running: true,
            selected_tab_index: 0,
            resistance_input: Input::default(),
            tolerance_input: Input::default(),
            tcr_input: Input::default(),
            focus: InputFocus::default(),
            resistor: None,
            error: None,
            selected_band: 0,
            selected_in_bands: [1, 0, 0, 0, 1, 0],
        }
    }
}

fn tabs<'a>(selected: usize) -> Tabs<'a> {
    Tabs::new(vec!["color codes to specs", "specs to color codes"])
        .padding(" ", " ")
        .select(selected)
}

fn band_list<'a>(number: usize, is_focused: bool) -> List<'a> {
    let items = [
        rusistor::Color::Black,
        rusistor::Color::Brown,
        rusistor::Color::Red,
        rusistor::Color::Orange,
        rusistor::Color::Yellow,
        rusistor::Color::Green,
        rusistor::Color::Blue,
        rusistor::Color::Violet,
        rusistor::Color::Grey,
        rusistor::Color::White,
        rusistor::Color::Gold,
        rusistor::Color::Silver,
        rusistor::Color::Pink,
    ]
    .iter()
    .map(|color| {
        let (c, s) = rusistor_color_to_ratatui_color(color);
        ListItem::new(s).bg(c)
    });

    let style = if is_focused {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    List::new(items)
        .block(
            Block::bordered()
                .title(format!(" Band {number} "))
                .style(style),
        )
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

pub fn view(model: &Model, frame: &mut Frame) {
    if model.selected_tab_index == 0 {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(2),
                    Constraint::Max(3),
                    Constraint::Max(15),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(frame.area());
        let tabs_rect = chunks[0];

        let spec_chuncks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ])
            .split(chunks[1]);

        let bands_rect = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
                Constraint::Ratio(1, 6),
            ])
            .split(chunks[2]);

        let tabs = tabs(model.selected_tab_index);
        frame.render_widget(tabs, tabs_rect);

        let resistor = match rusistor::Resistor::try_create(vec![
            (index_to_color(model.selected_in_bands[0])),
            (index_to_color(model.selected_in_bands[1])),
            (index_to_color(model.selected_in_bands[2])),
            (index_to_color(model.selected_in_bands[3])),
            (index_to_color(model.selected_in_bands[4])),
            (index_to_color(model.selected_in_bands[5])),
        ]) {
            Ok(r) => {
                let specs = r.specs();
                let s0 = format!("{}Ω", specs.ohm);
                let s1 = format!("±{}%", (specs.tolerance * 100.0));
                let s2 = format!("{}Ω", specs.min_ohm);
                let s3 = format!("{}Ω", specs.max_ohm);
                let s4 = format!(
                    "{}(ppm/K)",
                    specs.tcr.map(|f| f.to_string()).unwrap_or_default()
                );
                (s0, s1, s2, s3, s4)
            }
            Err(e) => (
                e.to_string(),
                e.to_string(),
                e.to_string(),
                e.to_string(),
                e.to_string(),
            ),
        };

        let resistance_paragraph = Paragraph::new(resistor.0)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Resistance(Ω) "),
            );
        frame.render_widget(resistance_paragraph, spec_chuncks[0]);

        let tolerance_paragraph = Paragraph::new(resistor.1)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Tolerance(Ω) "),
            );
        frame.render_widget(tolerance_paragraph, spec_chuncks[1]);

        let min_paragraph = Paragraph::new(resistor.2)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" Minimum(Ω) "));
        frame.render_widget(min_paragraph, spec_chuncks[2]);

        let max_paragraph = Paragraph::new(resistor.3)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" Maximum(Ω) "));
        frame.render_widget(max_paragraph, spec_chuncks[3]);

        let tcr_paragraph = Paragraph::new(resistor.4)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title(" TCR(ppm/K) "));
        frame.render_widget(tcr_paragraph, spec_chuncks[4]);

        for i in 0..model.selected_in_bands.len() {
            let mut state = ListState::default().with_selected(Some(model.selected_in_bands[i]));
            let is_focused = model.selected_band == i;
            let list = band_list(i + 1, is_focused);
            frame.render_stateful_widget(list, bands_rect[i], &mut state);
        }
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(2),
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
            .split(chunks[2]);

        let main_rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(chunks[3]);

        let tabs_rect = chunks[0];
        let help_msg_rect = chunks[1];
        let resistance_rect = input_rects[0];
        let tolerance_rect = input_rects[1];
        let tcr_rect = input_rects[2];
        let main_rect = main_rects[1];

        let tabs = tabs(model.selected_tab_index);
        frame.render_widget(tabs, tabs_rect);

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
}

pub enum Msg {
    ToggleTab,
    Determine,
    NextSpecInput,
    PrevSpecInput,
    NextBand,
    PrevBand,
    NextColor,
    PrevColor,
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
        KeyCode::Right | KeyCode::Left => Some(Msg::ToggleTab),
        KeyCode::Enter => Some(Msg::Determine),
        KeyCode::Tab if model.selected_tab_index == 0 => Some(Msg::NextSpecInput),
        KeyCode::Tab if model.selected_tab_index == 1 => Some(Msg::NextBand),
        KeyCode::BackTab if model.selected_tab_index == 0 => Some(Msg::PrevSpecInput),
        KeyCode::BackTab if model.selected_tab_index == 1 => Some(Msg::PrevBand),
        KeyCode::Up if model.selected_tab_index == 1 => Some(Msg::PrevColor),
        KeyCode::Down if model.selected_tab_index == 1 => Some(Msg::NextColor),
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
        Msg::ToggleTab => model.selected_tab_index = (model.selected_tab_index + 1) % 2,
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
        Msg::NextSpecInput | Msg::PrevSpecInput => {
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
                model.focus = match msg {
                    Msg::NextSpecInput => model.focus.next(),
                    _ => model.focus.prev(),
                };
            } else {
                model.resistor = None;
            }
        }
        Msg::Exit => {
            model.running = false;
        }
        Msg::NextBand => model.selected_band = (model.selected_band + 1) % 6,
        Msg::PrevBand => model.selected_band = (model.selected_band + 5) % 6,
        Msg::NextColor => {
            model.selected_in_bands[model.selected_band] =
                (model.selected_in_bands[model.selected_band] + 1) % 13;
        }
        Msg::PrevColor => {
            model.selected_in_bands[model.selected_band] =
                (model.selected_in_bands[model.selected_band] + 12) % 13;
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
        rusistor::Color::Black => (Color::Black, rusistor::Color::Black.to_string()),
        rusistor::Color::Brown => (Color::Rgb(165, 42, 42), rusistor::Color::Brown.to_string()),
        rusistor::Color::Red => (Color::Red, rusistor::Color::Red.to_string()),
        rusistor::Color::Orange => (Color::Rgb(255, 165, 0), rusistor::Color::Orange.to_string()),
        rusistor::Color::Yellow => (Color::Yellow, rusistor::Color::Yellow.to_string()),
        rusistor::Color::Green => (Color::Green, rusistor::Color::Green.to_string()),
        rusistor::Color::Blue => (Color::Blue, rusistor::Color::Blue.to_string()),
        rusistor::Color::Violet => (Color::Rgb(148, 0, 211), rusistor::Color::Violet.to_string()),
        rusistor::Color::Grey => (Color::Gray, rusistor::Color::Grey.to_string()),
        rusistor::Color::White => (Color::White, rusistor::Color::White.to_string()),
        rusistor::Color::Gold => (Color::Rgb(255, 215, 0), rusistor::Color::Gold.to_string()),
        rusistor::Color::Silver => (
            Color::Rgb(192, 192, 192),
            rusistor::Color::Silver.to_string(),
        ),
        rusistor::Color::Pink => (Color::Rgb(255, 105, 180), rusistor::Color::Pink.to_string()),
    }
}

fn index_to_color(idx: usize) -> rusistor::Color {
    match idx {
        0 => rusistor::Color::Black,
        1 => rusistor::Color::Brown,
        2 => rusistor::Color::Red,
        3 => rusistor::Color::Orange,
        4 => rusistor::Color::Yellow,
        5 => rusistor::Color::Green,
        6 => rusistor::Color::Blue,
        7 => rusistor::Color::Violet,
        8 => rusistor::Color::Grey,
        9 => rusistor::Color::White,
        10 => rusistor::Color::Gold,
        11 => rusistor::Color::Silver,
        12 => rusistor::Color::Pink,
        _ => panic!("unknown color"),
    }
}
