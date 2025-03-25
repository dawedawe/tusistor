use tusistor_core::model::{ColorCodesToSpecsModel, SelectedTab, SpecsToColorModel};

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
