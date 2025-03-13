use rusistor::Resistor;

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
