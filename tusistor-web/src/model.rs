use tusistor_core::model::{ColorCodesToSpecsModel, SelectedTab, SpecsToColorModel};

#[derive(Debug, Default)]
pub struct Model<'a> {
    pub selected_tab: SelectedTab,
    pub specs_to_color: SpecsToColorModel<'a>,
    pub color_codes_to_specs: ColorCodesToSpecsModel,
}
