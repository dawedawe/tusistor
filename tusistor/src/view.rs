use crate::model::Model;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Bar, BarChart, BarGroup, Block, Borders, List, ListDirection, ListItem, ListState, Padding,
        Paragraph, Tabs,
    },
};
use tusistor_core::{
    model::{InputFocus, SelectedTab},
    view::{band_numeric_info, band_semantic_info},
};

const BAR_WIDTH: u16 = 19;

fn tabs<'a>(selected: &SelectedTab) -> Tabs<'a> {
    Tabs::new(vec![" color codes to specs ", " specs to color codes "])
        .padding(" ", " ")
        .divider(symbols::DOT)
        .select(selected)
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
    let specs_style = Style::default().fg(Color::Yellow);

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
                .style(specs_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Resistance (Ω) "),
                );
            frame.render_widget(resistance_paragraph, spec_chuncks[0]);

            let tolerance_paragraph = Paragraph::new(format!("±{}", (specs.tolerance * 100.0)))
                .style(specs_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Tolerance (%) "),
                );
            frame.render_widget(tolerance_paragraph, spec_chuncks[1]);

            let min_paragraph = Paragraph::new(specs.min_ohm.to_string())
                .style(specs_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Minimum (Ω) "),
                );
            frame.render_widget(min_paragraph, spec_chuncks[2]);

            let max_paragraph = Paragraph::new(specs.max_ohm.to_string())
                .style(specs_style)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" Maximum (Ω) "),
                );
            frame.render_widget(max_paragraph, spec_chuncks[3]);

            let tcr_paragraph =
                Paragraph::new(specs.tcr.map(|f| f.to_string()).unwrap_or_default())
                    .style(specs_style)
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
                    .style(specs_style)
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
            let tolerance_paragraph = Paragraph::new(model.specs_to_color.tolerance_input.value())
                .style(specs_style)
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
                .style(specs_style)
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
                let chart_length: u16 = {
                    let bands_len: u16 = (bands.len() as u16).clamp(2, 6); // give title enough space
                    let bands_widths = bands_len * BAR_WIDTH;
                    let bands_gaps = bands_len - 1;
                    let border_plus_margin = 4;
                    bands_widths + bands_gaps + border_plus_margin
                };
                let centered_main_rect = center_horizontal(main_rect, chart_length);
                frame.render_widget(chart, centered_main_rect);
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
        " Resistance: {}Ω - Tolerance: ±{}%{} ",
        ohm,
        tolerance * 100.0,
        tcr
    );
    let title = Line::from(title).centered();
    BarChart::default()
        .data(BarGroup::default().bars(&bars))
        .block(
            Block::new()
                .padding(Padding::new(1, 1, 1, 1))
                .title(title)
                .borders(Borders::all()),
        )
        .bar_width(BAR_WIDTH)
        .bar_gap(1)
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
