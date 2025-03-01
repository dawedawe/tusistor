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
