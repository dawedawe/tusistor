use ratzilla::{
    event::KeyCode,
    ratatui::{
        layout::Rect,
        widgets::{Paragraph, Widget},
    },
};

#[derive(Default, Debug, Clone)]
pub struct WebInput {
    value: String,
    cursor: usize,
}
impl WebInput {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn visual_cursor(&self) -> usize {
        self.value.len()
    }

    pub fn visual_scroll(&self, width: usize) -> usize {
        let scroll = (self.visual_cursor()).max(width) - width;
        let mut uscroll = 0;
        let mut chars = self.value().chars();

        while uscroll < scroll {
            match chars.next() {
                Some(_) => {
                    uscroll += 1;
                }
                None => break,
            }
        }
        uscroll
    }

    pub fn handle_event(&mut self, event: &ratzilla::event::KeyEvent) {
        match event.code {
            KeyCode::Char(ch) if char::is_ascii(&ch) => {
                if self.cursor == self.value.len() {
                    self.value.push(ch);
                } else {
                    self.value.insert(self.cursor, ch);
                }
                self.cursor += 1
            }
            KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
            KeyCode::Right => self.cursor = self.cursor.saturating_add(1).min(self.value.len()),
            KeyCode::Delete => {
                if self.value.len() > self.cursor {
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Backspace => {
                let idx = self.cursor.saturating_sub(1);
                if self.cursor > 0 && self.value.len() > idx {
                    self.value.remove(idx);
                }
                self.cursor = self.cursor.saturating_sub(1);
            }
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.value.clear();
        self.cursor = 0;
    }
}

impl Widget for WebInput {
    fn render(self, area: Rect, buf: &mut ratzilla::ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let input = Paragraph::new(self.value);
        input.render(area, buf);
    }
}
