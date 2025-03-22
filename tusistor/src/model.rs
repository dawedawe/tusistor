use rusistor::Resistor;
use tui_input::Input;
use tusistor_core::model::{ColorCodesToSpecsModel, InputFocus, SelectedTab};

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
