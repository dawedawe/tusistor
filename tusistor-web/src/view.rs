use crate::model::Model;
use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, ListState, Paragraph},
};
use tusistor_core::{
    model::{InputFocus, SelectedTab},
    view::{
        BAR_WIDTH, band_list, band_numeric_info, band_semantic_info, barchart,
        rusistor_color_to_ratatui_color, tabs,
    },
};

pub fn view(model: &mut Model, frame: &mut Frame) {
    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    fn apply_title<'a>(
        block: Block<'a>,
        current_focus: &InputFocus,
        input: InputFocus,
        title: &str,
        title_style: Style,
    ) -> Block<'a> {
        if *current_focus == input {
            block
                .title(format!("{}* ", title))
                .title_style(title_style.bold())
        } else {
            block.title(format!("{} ", title))
        }
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
                    Span::styled("←/→", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": prev/next band, "),
                    Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": prev/next color, "),
                    Span::styled("3|4|5|6", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": bands count, "),
                    Span::styled("Shift ←/→", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": prev/next tab"),
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
            let help_msg_rect = center_horizontal(chunks[3], 115);
            let resistance_rect = input_rects[0];
            let tolerance_rect = input_rects[1];
            let tcr_rect = input_rects[2];
            let main_rect = chunks[2];

            let tabs = tabs(&model.selected_tab);
            frame.render_widget(tabs, tabs_rect);

            let (msg, style) = (
                vec![
                    Span::styled("Ctrl ←/→", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": prev/next input, "),
                    Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": calculate color codes, "),
                    Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" prev/next history, "),
                    Span::styled("X", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": reset, "),
                    Span::styled("Shift ←/→", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(": prev/next tab"),
                ],
                Style::default(),
            );
            let text = Text::from(Line::from(msg)).style(style);
            let help_message = Paragraph::new(text);
            frame.render_widget(help_message, help_msg_rect);

            // render resistance input
            let resistance_block = Block::default().borders(Borders::ALL).style(specs_style);
            let resistance_block = apply_title(
                resistance_block,
                &model.specs_to_color.focus,
                InputFocus::Resistance,
                " Resistance (Ω)",
                specs_style,
            );
            model
                .specs_to_color
                .resistance_textarea
                .set_block(resistance_block);
            model
                .specs_to_color
                .resistance_textarea
                .set_cursor_line_style(specs_style);
            frame.render_widget(&model.specs_to_color.resistance_textarea, resistance_rect);

            // render tolerance input
            let tolerance_block = Block::default().borders(Borders::ALL).style(specs_style);
            let tolerance_block = apply_title(
                tolerance_block,
                &model.specs_to_color.focus,
                InputFocus::Tolerance,
                " Tolerance (%)",
                specs_style,
            );
            model
                .specs_to_color
                .tolerance_textarea
                .set_block(tolerance_block);
            model
                .specs_to_color
                .tolerance_textarea
                .set_cursor_line_style(specs_style);
            frame.render_widget(&model.specs_to_color.tolerance_textarea, tolerance_rect);

            // render TCR input
            let tcr_block = Block::default().borders(Borders::ALL).style(specs_style);
            let tcr_block = apply_title(
                tcr_block,
                &model.specs_to_color.focus,
                InputFocus::Tcr,
                " TCR (ppm/K)",
                specs_style,
            );
            model.specs_to_color.tcr_textarea.set_block(tcr_block);
            model
                .specs_to_color
                .tcr_textarea
                .set_cursor_line_style(specs_style);
            frame.render_widget(&model.specs_to_color.tcr_textarea, tcr_rect);

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
                let centered_main_rect = center_horizontal(main_rect, e.len() as u16);
                frame.render_widget(error_message, centered_main_rect);
            }
        }
    }
}
