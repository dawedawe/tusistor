use rusistor::Resistor;
use tui_input::Input;
use tusistor_core::model::{ColorCodesToSpecsModel, InputFocus, SelectedTab, SpecsHistory};

#[derive(Debug, Default)]
pub struct SpecsToColorModel {
    pub resistance_input: Input,
    pub tolerance_input: Input,
    pub tcr_input: Input,
    pub focus: InputFocus,
    pub resistor: Option<Resistor>,
    pub history: SpecsHistory,
    pub error: Option<String>,
}

impl SpecsToColorModel {
    pub fn add_specs_to_history(&mut self) {
        let specs = (
            self.resistance_input.value().to_string(),
            self.tolerance_input.value().to_string(),
            self.tcr_input.value().to_string(),
        );
        self.history.add(specs);
    }

    pub fn set_specs_from_history(&mut self) {
        if let Some((a, b, c)) = self.history.try_get() {
            self.resistance_input = Input::new(a.to_string());
            self.tolerance_input = Input::new(b.to_string());
            self.tcr_input = Input::new(c.to_string());
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
