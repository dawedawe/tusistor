use ratzilla::ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};
use tusistor_core::model::SelectedTab;

pub(crate) fn help<'a>(selected_tab: &SelectedTab) -> Paragraph<'a> {
    match selected_tab {
        SelectedTab::ColorCodesToSpecs => {
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
            Paragraph::new(text)
        }
        SelectedTab::SpecsToColorCodes => {
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
            Paragraph::new(text)
        }
    }
}
