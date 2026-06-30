use tusistor_core::model::{ColorCodesToSpecsModel, SelectedTab, SpecsToColorModel};

#[derive(Debug)]
pub struct Model<'a> {
    pub running: bool,
    pub selected_tab: SelectedTab,
    pub specs_to_color: SpecsToColorModel<'a>,
    pub color_codes_to_specs: ColorCodesToSpecsModel,
}

impl<'a> Default for Model<'a> {
    fn default() -> Model<'a> {
        Model {
            running: true,
            selected_tab: SelectedTab::default(),
            specs_to_color: SpecsToColorModel::default(),
            color_codes_to_specs: ColorCodesToSpecsModel::default(),
        }
    }
}
