use tusistor_core::model::{ColorCodesToSpecsModel, SelectedTab, SpecsToColorModel};

#[derive(Debug, Default)]
pub struct Model {
    pub selected_tab: SelectedTab,
    pub specs_to_color: SpecsToColorModel,
    pub color_codes_to_specs: ColorCodesToSpecsModel,
}
