use rusistor::Resistor;
use tusistor_core::model::{ColorCodesToSpecsModel, InputFocus, SelectedTab};

use crate::input::WebInput;

#[derive(Debug, Default)]
pub struct SpecsToColorModel {
    pub resistance_input: WebInput,
    pub tolerance_input: WebInput,
    pub tcr_input: WebInput,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub error: Option<String>,
}

#[derive(Debug, Default)]
pub struct Model {
    pub selected_tab: SelectedTab,
    pub specs_to_color: SpecsToColorModel,
    pub color_codes_to_specs: ColorCodesToSpecsModel,
}
