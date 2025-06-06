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

#[derive(Debug, Default, PartialEq)]
pub struct InputState {
    pub value: String,
    pub cursor: usize,
}

#[derive(Debug, Default)]
pub struct SpecsToColorModel {
    pub resistance_input_state: InputState,
    pub tolerance_input_state: InputState,
    pub tcr_input_state: InputState,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub history: SpecsHistory,
    pub error: Option<String>,
}

impl SpecsToColorModel {
    pub fn add_specs_to_history(&mut self) {
        let specs = (
            self.resistance_input_state.value.clone(),
            self.tolerance_input_state.value.clone(),
            self.tcr_input_state.value.clone(),
        );
        self.history.add(specs);
    }

    pub fn set_specs_from_history(&mut self) {
        if let Some((a, b, c)) = self.history.try_get() {
            self.resistance_input_state = InputState {
                value: a.to_string(),
                cursor: a.len(),
            };
            self.tolerance_input_state = InputState {
                value: b.to_string(),
                cursor: b.len(),
            };
            self.tcr_input_state = InputState {
                value: c.to_string(),
                cursor: c.len(),
            };
        }
    }
}
