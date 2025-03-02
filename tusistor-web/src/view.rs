use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph},
};
use tusistor_core::{
    model::ColorCodesToSpecsModel,
    view::{band_numeric_info, band_semantic_info},
};

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

pub fn view(model: &ColorCodesToSpecsModel, frame: &mut Frame) {
    fn center_horizontal(area: Rect, width: u16) -> Rect {
        let [area] = Layout::horizontal([Constraint::Length(width)])
            .flex(Flex::Center)
            .areas(area);
        area
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(15),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(frame.area());
    let help_msg_rect = center_horizontal(chunks[2], 95);

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
        .split(chunks[0]);

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
        .split(chunks[1]);

    let specs = model.resistor.specs();

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

    let tcr_paragraph = Paragraph::new(specs.tcr.map(|f| f.to_string()).unwrap_or_default())
        .style(Style::default().fg(Color::Yellow))
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
        ],
        Style::default(),
    );
    let text = Text::from(Line::from(msg)).style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_msg_rect);

    let bands = model.resistor.bands();
    for i in 0..bands.len() {
        let mut state = ListState::default().with_selected(Some(*bands[i] as usize));
        let is_focused = model.selected_band == i;
        let list = band_list(i, bands.len(), is_focused);
        frame.render_stateful_widget(list, bands_rect[i], &mut state);
    }
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
