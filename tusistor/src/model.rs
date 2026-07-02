use tusistor_core::model::Model;

#[derive(Debug)]
pub struct TuiModel<'a> {
    pub running: bool,
    pub core_model: Model<'a>,
}

impl<'a> Default for TuiModel<'a> {
    fn default() -> TuiModel<'a> {
        TuiModel {
            running: true,
            core_model: Default::default(),
        }
    }
}
