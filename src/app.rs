pub mod model {
    use rusistor::Resistor;
    use tui_input::Input;

    #[derive(Debug, Default)]
    pub enum InputFocus {
        #[default]
        Resistance,
        Tolerance,
        Tcr,
    }

    impl InputFocus {
        pub fn next(&self) -> InputFocus {
            match self {
                InputFocus::Resistance => InputFocus::Tolerance,
                InputFocus::Tolerance => InputFocus::Tcr,
                InputFocus::Tcr => InputFocus::Resistance,
            }
        }

        pub fn prev(&self) -> InputFocus {
            match self {
                InputFocus::Resistance => InputFocus::Tcr,
                InputFocus::Tolerance => InputFocus::Resistance,
                InputFocus::Tcr => InputFocus::Tolerance,
            }
        }
    }

    #[derive(Debug, Default, PartialEq)]
    pub enum SelectedTab {
        #[default]
        ColorCodesToSpecs,
        SpecsToColorCodes,
    }

    impl SelectedTab {
        pub fn toggle(&self) -> SelectedTab {
            match self {
                SelectedTab::ColorCodesToSpecs => SelectedTab::SpecsToColorCodes,
                SelectedTab::SpecsToColorCodes => SelectedTab::ColorCodesToSpecs,
            }
        }
    }

    impl From<&SelectedTab> for Option<usize> {
        fn from(selected_tab: &SelectedTab) -> Self {
            match selected_tab {
                SelectedTab::ColorCodesToSpecs => Some(0),
                SelectedTab::SpecsToColorCodes => Some(1),
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct SpecsToColorModel {
        pub resistance_input: Input,
        pub tolerance_input: Input,
        pub tcr_input: Input,
        pub focus: InputFocus,
        pub resistor: Option<Resistor>,
        pub error: Option<String>,
    }

    #[derive(Debug)]
    pub struct ColorCodesToSpecsModel {
        pub selected_band: usize,
        pub resistor: Resistor,
    }

    impl Default for ColorCodesToSpecsModel {
        fn default() -> ColorCodesToSpecsModel {
            ColorCodesToSpecsModel {
                selected_band: 0,
                resistor: Resistor::SixBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                    band4: rusistor::Color::Black,
                    band5: rusistor::Color::Brown,
                    band6: rusistor::Color::Black,
                },
            }
        }
    }

    #[derive(Debug)]
    pub struct Model {
        pub running: bool,
        pub selected_tab: SelectedTab,
        pub specs_to_color: SpecsToColorModel,
        pub color_codes_to_specs: ColorCodesToSpecsModel,
    }

    impl Default for Model {
        fn default() -> Model {
            Model {
                running: true,
                selected_tab: SelectedTab::default(),
                specs_to_color: SpecsToColorModel::default(),
                color_codes_to_specs: ColorCodesToSpecsModel::default(),
            }
        }
    }
}

pub mod view {
    use crate::app::model::{InputFocus, Model, SelectedTab};
    use ratatui::{
        layout::{Constraint, Direction, Flex, Layout, Rect},
        style::{Color, Modifier, Style},
        symbols,
        text::{Line, Span, Text},
        widgets::{
            Bar, BarChart, BarGroup, Block, Borders, List, ListDirection, ListItem, ListState,
            Paragraph, Tabs,
        },
        Frame,
    };

    fn tabs<'a>(selected: &SelectedTab) -> Tabs<'a> {
        Tabs::new(vec![" color codes to specs ", " specs to color codes "])
            .padding(" ", " ")
            .divider(symbols::DOT)
            .select(selected)
    }

    fn band_numeric_info(bands: usize, band_idx: usize, color: &rusistor::Color) -> String {
        match (bands, band_idx) {
            (3, i) | (4, i) if i <= 1 => {
                if i == 0 && *color == rusistor::Color::Black {
                    " ".to_string()
                } else {
                    color.as_digit().map_or(" ".to_string(), |s| s.to_string())
                }
            }
            (5, i) | (6, i) if i <= 2 => {
                if i == 0 && *color == rusistor::Color::Black {
                    " ".to_string()
                } else {
                    color.as_digit().map_or(" ".to_string(), |s| s.to_string())
                }
            }
            (3, 2) | (4, 2) | (5, 3) | (6, 3) => {
                format!("10^{}", color.as_digit_or_exponent())
            }
            (4, 3) | (5, 4) | (6, 4) => color
                .as_tolerance()
                .map_or("    ".to_string(), |s| format!("{:>4}", (s * 100.0))),
            (6, 5) => color
                .as_tcr()
                .map_or("   ".to_string(), |s| format!("{:>3}", s.to_string())),
            _ => "".to_string(),
        }
    }

    fn band_semantic_info(bands: usize, band_idx: usize) -> String {
        match (bands, band_idx) {
            (3, i) | (4, i) if i <= 1 => format!("Digit {}", band_idx + 1),
            (5, i) | (6, i) if i <= 2 => format!("Digit {}", band_idx + 1),
            (3, 2) | (4, 2) | (5, 3) | (6, 3) => "Multiplier".to_string(),
            (4, 3) | (5, 4) | (6, 4) => "Tolerance".to_string(),
            (6, 5) => "TCR".to_string(),
            _ => "".to_string(),
        }
    }

    fn band_list<'a>(band_idx: usize, bands: usize, is_focused: bool) -> List<'a> {
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
            let numeric_info = band_numeric_info(bands, band_idx, color);
            let (color, name) = rusistor_color_to_ratatui_color(color);
            let s = format!(" {numeric_info} {name}");
            let style = if color == Color::Black {
                Style::default().bg(color)
            } else {
                Style::default().bg(color).fg(Color::Black)
            };
            ListItem::new(s).style(style)
        });

        let style = if is_focused {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let semantic_info = band_semantic_info(bands, band_idx);

        List::new(items)
            .block(
                Block::bordered()
                    .title(format!(
                        " Band {}: {}{}",
                        band_idx + 1,
                        semantic_info,
                        if is_focused { "* " } else { " " }
                    ))
                    .style(style),
            )
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom)
    }

    pub fn view(model: &Model, frame: &mut Frame) {
        fn center_horizontal(area: Rect, width: u16) -> Rect {
            let [area] = Layout::horizontal([Constraint::Length(width)])
                .flex(Flex::Center)
                .areas(area);
            area
        }

        let tabs_width = 49;

        match model.selected_tab {
            SelectedTab::ColorCodesToSpecs => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(2),
                            Constraint::Length(3),
                            Constraint::Length(15),
                            Constraint::Min(1),
                        ]
                        .as_ref(),
                    )
                    .split(frame.area());
                let tabs_rect = center_horizontal(chunks[0], tabs_width);
                let help_msg_rect = center_horizontal(chunks[3], 95);

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

                let tabs = tabs(&model.selected_tab);
                frame.render_widget(tabs, tabs_rect);

                let specs = model.color_codes_to_specs.resistor.specs();

                let resistance_paragraph = Paragraph::new(specs.ohm.to_string())
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Resistance (Ω) "),
                    );
                frame.render_widget(resistance_paragraph, spec_chuncks[0]);

                let tolerance_paragraph = Paragraph::new(format!("±{}", (specs.tolerance * 100.0)))
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Tolerance (%) "),
                    );
                frame.render_widget(tolerance_paragraph, spec_chuncks[1]);

                let min_paragraph = Paragraph::new(specs.min_ohm.to_string())
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Minimum (Ω) "),
                    );
                frame.render_widget(min_paragraph, spec_chuncks[2]);

                let max_paragraph = Paragraph::new(specs.max_ohm.to_string())
                    .style(Style::default().fg(Color::Yellow))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" Maximum (Ω) "),
                    );
                frame.render_widget(max_paragraph, spec_chuncks[3]);

                let tcr_paragraph =
                    Paragraph::new(specs.tcr.map(|f| f.to_string()).unwrap_or_default())
                        .style(Style::default().fg(Color::Yellow))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" TCR (ppm/K) "),
                        );
                frame.render_widget(tcr_paragraph, spec_chuncks[4]);

                let (msg, style) = (
                    vec![
                        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": next band, "),
                        Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": prev/next color, "),
                        Span::styled("3|4|5|6", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": bands count, "),
                        Span::styled("Shift ←/→", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": prev/next tab, "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": exit"),
                    ],
                    Style::default(),
                );
                let text = Text::from(Line::from(msg)).style(style);
                let help_message = Paragraph::new(text);
                frame.render_widget(help_message, help_msg_rect);

                let bands = model.color_codes_to_specs.resistor.bands();
                for i in 0..bands.len() {
                    let mut state = ListState::default().with_selected(Some(*bands[i] as usize));
                    let is_focused = model.color_codes_to_specs.selected_band == i;
                    let list = band_list(i, bands.len(), is_focused);
                    frame.render_stateful_widget(list, bands_rect[i], &mut state);
                }
            }
            SelectedTab::SpecsToColorCodes => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints(
                        [
                            Constraint::Length(2),
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(1),
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

                let tabs_rect = center_horizontal(chunks[0], tabs_width);
                let help_msg_rect = center_horizontal(chunks[3], 82);
                let resistance_rect = input_rects[0];
                let tolerance_rect = input_rects[1];
                let tcr_rect = input_rects[2];
                let main_rect = chunks[2];

                let tabs = tabs(&model.selected_tab);
                frame.render_widget(tabs, tabs_rect);

                let (msg, style) = (
                    vec![
                        Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": next input, "),
                        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": calculate color codes, "),
                        Span::styled("Shift ←/→", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": prev/next tab, "),
                        Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(": exit"),
                    ],
                    Style::default(),
                );
                let text = Text::from(Line::from(msg)).style(style);
                let help_message = Paragraph::new(text);
                frame.render_widget(help_message, help_msg_rect);

                let resistance_width = resistance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
                let resistance_scroll = model
                    .specs_to_color
                    .resistance_input
                    .visual_scroll(resistance_width as usize);
                let resistance_paragraph =
                    Paragraph::new(model.specs_to_color.resistance_input.value())
                        .style(Style::default().fg(Color::Yellow))
                        .scroll((0, resistance_scroll as u16))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" Resistance (Ω) "),
                        );
                frame.render_widget(resistance_paragraph, resistance_rect);

                let tolerance_width = tolerance_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
                let tolerance_scroll = model
                    .specs_to_color
                    .tolerance_input
                    .visual_scroll(tolerance_width as usize);
                let tolerance_paragraph =
                    Paragraph::new(model.specs_to_color.tolerance_input.value())
                        .style(Style::default().fg(Color::Yellow))
                        .scroll((0, tolerance_scroll as u16))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(" Tolerance (%) "),
                        );
                frame.render_widget(tolerance_paragraph, tolerance_rect);

                let tcr_width = tcr_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor
                let tcr_scroll = model
                    .specs_to_color
                    .tcr_input
                    .visual_scroll(tcr_width as usize);
                let tcr_paragraph = Paragraph::new(model.specs_to_color.tcr_input.value())
                    .style(Style::default().fg(Color::Yellow))
                    .scroll((0, tcr_scroll as u16))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title(" TCR (ppm/K) "),
                    );
                frame.render_widget(tcr_paragraph, tcr_rect);

                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                let (rect, input, scroll) = match model.specs_to_color.focus {
                    InputFocus::Resistance => (
                        resistance_rect,
                        &model.specs_to_color.resistance_input,
                        resistance_scroll,
                    ),
                    InputFocus::Tolerance => (
                        tolerance_rect,
                        &model.specs_to_color.tolerance_input,
                        tolerance_scroll,
                    ),
                    InputFocus::Tcr => (tcr_rect, &model.specs_to_color.tcr_input, tcr_scroll),
                };
                frame.set_cursor_position((
                    // Put cursor past the end of the input text
                    rect.x + ((input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                    // Move one line down, from the border to the input line
                    rect.y + 1,
                ));

                if let Some(resistor) = &model.specs_to_color.resistor {
                    let bands = resistor.bands();
                    let band_infos = bands
                        .iter()
                        .enumerate()
                        .map(|(idx, c)| {
                            let sem_info = band_semantic_info(bands.len(), idx);
                            let num_info = band_numeric_info(bands.len(), idx, c);
                            let (color, name) = rusistor_color_to_ratatui_color(c);
                            (sem_info, num_info, color, name)
                        })
                        .collect::<Vec<(String, String, Color, String)>>();
                    let specs = resistor.specs();
                    let chart = barchart(&band_infos, specs.ohm, specs.tolerance, specs.tcr);
                    frame.render_widget(chart, main_rect);
                }
                if let Some(e) = &model.specs_to_color.error {
                    let text = Text::from(e.to_string());
                    let error_message = Paragraph::new(text).style(Style::default().fg(Color::Red));
                    frame.render_widget(error_message, main_rect);
                }
            }
        }
    }

    fn barchart(
        band_infos: &[(String, String, Color, String)],
        ohm: f64,
        tolerance: f64,
        tcr: Option<u32>,
    ) -> BarChart {
        let bars: Vec<Bar> = band_infos.iter().map(|i| bar(i)).collect();
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
            .bar_width(19)
    }

    fn bar((sem_info, num_info, color, name): &(String, String, Color, String)) -> Bar {
        Bar::default()
            .value(100)
            .text_value(format!(" {} ", name))
            .value_style(Style::default().fg(Color::White).bg(Color::Black))
            .label(Line::from(format!("{}: {}", sem_info, num_info.trim())))
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
            rusistor::Color::Orange => {
                (Color::Rgb(255, 165, 0), rusistor::Color::Orange.to_string())
            }
            rusistor::Color::Yellow => (Color::Yellow, rusistor::Color::Yellow.to_string()),
            rusistor::Color::Green => (Color::Green, rusistor::Color::Green.to_string()),
            rusistor::Color::Blue => (Color::Blue, rusistor::Color::Blue.to_string()),
            rusistor::Color::Violet => {
                (Color::Rgb(148, 0, 211), rusistor::Color::Violet.to_string())
            }
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
}

pub mod update {
    use crate::app::model::{InputFocus, Model, SelectedTab};
    use color_eyre::Result;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
    use rusistor::{self, Resistor};
    use std::str::FromStr;
    use tui_input::backend::crossterm::EventHandler;

    pub enum ColorCodesMsg {
        ThreeBands,
        FourBands,
        FiveBands,
        SixBands,
        NextBand,
        PrevBand,
        NextColor,
        PrevColor,
    }

    pub enum SpecsMsg {
        Determine,
        NextSpecInput,
        PrevSpecInput,
    }

    pub enum Msg {
        ToggleTab,
        Exit,
        SpecsMsg { msg: SpecsMsg },
        ColorCodesMsg { msg: ColorCodesMsg },
    }

    pub fn handle_event(model: &mut Model) -> Result<Option<Msg>> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                Result::Ok(on_key_event(model, key))
            }
            _ => Result::Ok(None),
        }
    }

    fn on_key_event(model: &mut Model, key: KeyEvent) -> Option<Msg> {
        match (key.code, &model.selected_tab) {
            (KeyCode::Left, _) | (KeyCode::Right, _) if key.modifiers == KeyModifiers::SHIFT => {
                Some(Msg::ToggleTab)
            }
            (KeyCode::Enter, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
                msg: SpecsMsg::Determine,
            }),
            (KeyCode::Tab, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
                msg: SpecsMsg::NextSpecInput,
            }),
            (KeyCode::Tab, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextBand,
            }),
            (KeyCode::BackTab, SelectedTab::SpecsToColorCodes) => Some(Msg::SpecsMsg {
                msg: SpecsMsg::PrevSpecInput,
            }),
            (KeyCode::BackTab, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevBand,
            }),
            (KeyCode::Up, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevColor,
            }),
            (KeyCode::Down, SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextColor,
            }),
            (KeyCode::Char('3'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::ThreeBands,
            }),
            (KeyCode::Char('4'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FourBands,
            }),
            (KeyCode::Char('5'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FiveBands,
            }),
            (KeyCode::Char('6'), SelectedTab::ColorCodesToSpecs) => Some(Msg::ColorCodesMsg {
                msg: ColorCodesMsg::SixBands,
            }),
            (KeyCode::Esc, _) => Some(Msg::Exit),
            _ => {
                let target_input = match model.specs_to_color.focus {
                    InputFocus::Resistance => &mut model.specs_to_color.resistance_input,
                    InputFocus::Tolerance => &mut model.specs_to_color.tolerance_input,
                    InputFocus::Tcr => &mut model.specs_to_color.tcr_input,
                };
                target_input.handle_event(&Event::Key(key));
                None
            }
        }
    }

    pub fn update(model: &mut Model, msg: Msg) {
        match msg {
            Msg::ToggleTab => model.selected_tab = model.selected_tab.toggle(),
            Msg::SpecsMsg {
                msg: SpecsMsg::Determine,
            } => {
                match try_determine_resistor(
                    model.specs_to_color.resistance_input.value(),
                    model.specs_to_color.tolerance_input.value(),
                    model.specs_to_color.tcr_input.value(),
                ) {
                    Ok(resistor) => {
                        model.specs_to_color.resistor = Some(resistor);
                        model.specs_to_color.error = None;
                    }
                    Err(e) => {
                        model.specs_to_color.resistor = None;
                        model.specs_to_color.error = Some(e);
                    }
                }
                model.specs_to_color.resistance_input.reset();
                model.specs_to_color.tolerance_input.reset();
                model.specs_to_color.tcr_input.reset();
                model.specs_to_color.focus = InputFocus::Resistance;
            }
            Msg::SpecsMsg {
                msg: SpecsMsg::NextSpecInput,
            }
            | Msg::SpecsMsg {
                msg: SpecsMsg::PrevSpecInput,
            } => {
                model.specs_to_color.error = match model.specs_to_color.focus {
                    InputFocus::Resistance => {
                        let value = model.specs_to_color.resistance_input.value();
                        if value.trim().is_empty() {
                            None
                        } else {
                            try_parse_resistance(value).err().map(|err| err.to_string())
                        }
                    }
                    InputFocus::Tolerance => {
                        let value = model.specs_to_color.tolerance_input.value();
                        if value.trim().is_empty() {
                            None
                        } else {
                            value.parse::<f64>().err().map(|err| err.to_string())
                        }
                    }
                    InputFocus::Tcr => {
                        let value = model.specs_to_color.tcr_input.value();
                        if value.trim().is_empty() {
                            None
                        } else {
                            value.parse::<u32>().err().map(|err| err.to_string())
                        }
                    }
                };
                if model.specs_to_color.error.is_none() {
                    model.specs_to_color.focus = match msg {
                        Msg::SpecsMsg {
                            msg: SpecsMsg::NextSpecInput,
                        } => model.specs_to_color.focus.next(),
                        _ => model.specs_to_color.focus.prev(),
                    };
                } else {
                    model.specs_to_color.resistor = None;
                }
            }
            Msg::Exit => {
                model.running = false;
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::ThreeBands,
            } => {
                model.color_codes_to_specs.resistor = Resistor::ThreeBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(2)
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FourBands,
            } => {
                model.color_codes_to_specs.resistor = Resistor::FourBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                    band4: rusistor::Color::Brown,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(3)
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FiveBands,
            } => {
                model.color_codes_to_specs.resistor = Resistor::FiveBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                    band4: rusistor::Color::Black,
                    band5: rusistor::Color::Brown,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(4)
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::SixBands,
            } => {
                model.color_codes_to_specs.resistor = Resistor::SixBand {
                    band1: rusistor::Color::Brown,
                    band2: rusistor::Color::Black,
                    band3: rusistor::Color::Black,
                    band4: rusistor::Color::Black,
                    band5: rusistor::Color::Brown,
                    band6: rusistor::Color::Black,
                };
                model.color_codes_to_specs.selected_band =
                    model.color_codes_to_specs.selected_band.min(5)
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextBand,
            } => {
                model.color_codes_to_specs.selected_band =
                    (model.color_codes_to_specs.selected_band + 1)
                        % model.color_codes_to_specs.resistor.bands().len()
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevBand,
            } => {
                let bands_count = model.color_codes_to_specs.resistor.bands().len();
                model.color_codes_to_specs.selected_band =
                    (model.color_codes_to_specs.selected_band + (bands_count - 1)) % bands_count
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::NextColor,
            } => {
                let current_idx: usize = *model.color_codes_to_specs.resistor.bands()
                    [model.color_codes_to_specs.selected_band]
                    as usize;
                let mut i: usize = 0;
                let mut resistor = Err("".to_string());
                while resistor.is_err() {
                    i += 1;
                    let next_color = index_to_color((current_idx + i) % 13);
                    resistor = model
                        .color_codes_to_specs
                        .resistor
                        .with_color(next_color, model.color_codes_to_specs.selected_band);
                }
                model.color_codes_to_specs.resistor = resistor.unwrap();
            }
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::PrevColor,
            } => {
                let current_idx = *model.color_codes_to_specs.resistor.bands()
                    [model.color_codes_to_specs.selected_band]
                    as usize;
                let mut i: usize = 13;
                let mut resistor = Err("".to_string());
                while resistor.is_err() {
                    i -= 1;
                    let next_color = index_to_color((current_idx + i) % 13);
                    resistor = model
                        .color_codes_to_specs
                        .resistor
                        .with_color(next_color, model.color_codes_to_specs.selected_band);
                }
                model.color_codes_to_specs.resistor = resistor.unwrap();
            }
        }
    }

    fn try_parse_resistance(input: &str) -> Result<f64, String> {
        match input.parse::<f64>() {
            Ok(t) => Ok(t),
            Err(e) => match engineering_repr::EngineeringQuantity::<i64>::from_str(input) {
                Ok(t) => {
                    let r: i64 = t.into();
                    let r = r as f64;
                    Ok(r)
                }
                Err(_) => Err(format!("invalid input for resistance: {}", e)),
            },
        }
    }

    fn try_determine_resistor(
        resistance_input: &str,
        tolerance_input: &str,
        tcr_input: &str,
    ) -> Result<Resistor, String> {
        let resistance = try_parse_resistance(resistance_input);
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
}

#[cfg(test)]
mod tests {
    use super::model::{Model, SelectedTab};
    use super::update::{update, ColorCodesMsg, Msg};

    #[test]
    fn test_exit_msg() {
        let mut model = Model::default();
        update(&mut model, Msg::Exit);
        assert!(!model.running)
    }

    #[test]
    fn test_toggletab_msg() {
        let mut model = Model::default();
        update(&mut model, Msg::ToggleTab);
        assert_eq!(model.selected_tab, SelectedTab::SpecsToColorCodes);
        update(&mut model, Msg::ToggleTab);
        assert_eq!(model.selected_tab, SelectedTab::ColorCodesToSpecs)
    }

    #[test]
    fn test_nbands_msg() {
        let mut model = Model::default();
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 6);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::ThreeBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 3);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FourBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 4);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::FiveBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 5);

        update(
            &mut model,
            Msg::ColorCodesMsg {
                msg: ColorCodesMsg::SixBands,
            },
        );
        assert_eq!(model.color_codes_to_specs.resistor.bands().len(), 6);
    }
}
