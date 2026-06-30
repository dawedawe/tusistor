use ratatui_textarea::{CursorMove, TextArea};
use rusistor::Resistor;

#[derive(Debug, Default)]
pub struct SpecsHistory {
    history: Vec<(String, String, String)>,
    idx: Option<usize>,
}

impl SpecsHistory {
    pub fn prev(&mut self) {
        if !self.history.is_empty() {
            if let Some(idx) = self.idx {
                self.idx = Some(idx.saturating_sub(1));
            } else {
                self.idx = Some(self.history.len() - 1)
            }
        }
    }

    pub fn next(&mut self) {
        if !self.history.is_empty() {
            if let Some(idx) = self.idx {
                self.idx = Some(idx.saturating_add(1).min(self.history.len() - 1));
            } else {
                self.idx = None;
            }
        }
    }

    pub fn try_get(&self) -> Option<&(String, String, String)> {
        if let Some(idx) = self.idx {
            self.history.get(idx)
        } else {
            None
        }
    }

    pub fn add(&mut self, specs: (String, String, String)) {
        match self.history.last() {
            Some(x) if *x == specs => (),
            _ => self.history.push(specs),
        }
    }

    pub fn clear_idx(&mut self) {
        self.idx = None;
    }
}

#[derive(Debug, Default, PartialEq)]
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

#[derive(Debug, Default)]
pub struct SpecsToColorModel<'a> {
    pub resistance_textarea: TextArea<'a>,
    pub tolerance_textarea: TextArea<'a>,
    pub tcr_textarea: TextArea<'a>,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub history: SpecsHistory,
    pub error: Option<String>,
}

pub(crate) fn set_textarea(textarea: &mut TextArea, content: String, cursormoves: Vec<CursorMove>) {
    textarea.select_all();
    textarea.cut();
    textarea.insert_str(content);
    cursormoves.iter().for_each(|m| textarea.move_cursor(*m));
}

impl<'a> SpecsToColorModel<'a> {
    pub fn add_specs_to_history(&mut self) {
        let specs = (
            self.resistance_textarea.lines()[0].clone(),
            self.tolerance_textarea.lines()[0].clone(),
            self.tcr_textarea.lines()[0].clone(),
        );
        self.history.add(specs);
    }

    pub fn set_specs_from_history(&mut self) {
        if let Some((a, b, c)) = self.history.try_get() {
            set_textarea(&mut self.resistance_textarea, a.to_string(), vec![]);
            set_textarea(&mut self.tolerance_textarea, b.to_string(), vec![]);
            set_textarea(&mut self.tcr_textarea, c.to_string(), vec![]);
        }
    }
}
