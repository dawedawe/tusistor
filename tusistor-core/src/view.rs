use ratatui_core::{
    style::{Color, Modifier, Style},
    symbols,
    text::Line,
};
use ratatui_widgets::{
    barchart::{Bar, BarChart, BarGroup},
    block::{Block, Padding},
    borders::Borders,
    list::{List, ListDirection, ListItem},
    tabs::Tabs,
};

use crate::model::SelectedTab;

pub const BAR_WIDTH: u16 = 19;

pub fn tabs<'a>(selected: &SelectedTab) -> Tabs<'a> {
    let highlight_style = Style::default().fg(Color::Black).bg(Color::White);
    Tabs::new(vec![" color codes to specs ", " specs to color codes "])
        .padding(" ", " ")
        .divider(symbols::DOT)
        .highlight_style(highlight_style)
        .select(selected)
}

pub fn band_list<'a>(band_idx: usize, bands: usize, is_focused: bool) -> List<'a> {
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

pub fn band_numeric_info(bands: usize, band_idx: usize, color: &rusistor::Color) -> String {
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

pub fn band_semantic_info(bands: usize, band_idx: usize) -> String {
    match (bands, band_idx) {
        (1, 0) => format!("Digit {}", band_idx + 1),
        (3, i) | (4, i) if i <= 1 => format!("Digit {}", band_idx + 1),
        (5, i) | (6, i) if i <= 2 => format!("Digit {}", band_idx + 1),
        (3, 2) | (4, 2) | (5, 3) | (6, 3) => "Multiplier".to_string(),
        (4, 3) | (5, 4) | (6, 4) => "Tolerance".to_string(),
        (6, 5) => "TCR".to_string(),
        _ => "".to_string(),
    }
}

pub fn rusistor_color_to_ratatui_color(color: &rusistor::Color) -> (Color, String) {
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

pub fn barchart(
    band_infos: &[(String, String, Color, String)],
    ohm: f64,
    tolerance: f64,
    tcr: Option<u32>,
) -> BarChart<'_> {
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

fn bar((sem_info, num_info, color, name): &(String, String, Color, String)) -> Bar<'_> {
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
