use rusistor::Resistor;
use tusistor_core::model::{ColorCodesToSpecsModel, InputFocus, SelectedTab, SpecsHistory};

use crate::input::WebInput;

#[derive(Debug, Default)]
pub struct SpecsToColorModel {
    pub resistance_input: WebInput,
    pub tolerance_input: WebInput,
    pub tcr_input: WebInput,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub history: SpecsHistory,
    pub error: Option<String>,
}

impl SpecsToColorModel {
    pub(crate) fn add_specs_to_history(&mut self) {
        self.history.add((
            self.resistance_input.value().to_string(),
            self.tolerance_input.value().to_string(),
            self.tcr_input.value().to_string(),
        ));
    }

    pub fn set_specs_from_history(&mut self) {
        if let Some((a, b, c)) = self.history.try_get() {
            self.resistance_input = WebInput::new(a.to_string());
            self.tolerance_input = WebInput::new(b.to_string());
            self.tcr_input = WebInput::new(c.to_string());
        }
    }
}

#[derive(Debug, Default)]
pub struct Model {
    pub selected_tab: SelectedTab,
    pub specs_to_color: SpecsToColorModel,
    pub color_codes_to_specs: ColorCodesToSpecsModel,
}
