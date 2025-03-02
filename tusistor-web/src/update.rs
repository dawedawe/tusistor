use ratzilla::event;
use rusistor::{self, Resistor};
use tusistor_core::model::ColorCodesToSpecsModel;
use tusistor_core::update::{ColorCodesMsg, index_to_color};

pub fn handle_event(model: &mut ColorCodesToSpecsModel, event: ratzilla::event::KeyEvent) {
    match event.code {
        event::KeyCode::Char('3') => update(model, ColorCodesMsg::ThreeBands),
        event::KeyCode::Char('4') => update(model, ColorCodesMsg::FourBands),
        event::KeyCode::Char('5') => update(model, ColorCodesMsg::FiveBands),
        event::KeyCode::Char('6') => update(model, ColorCodesMsg::SixBands),
        event::KeyCode::Up => update(model, ColorCodesMsg::PrevColor),
        event::KeyCode::Down => update(model, ColorCodesMsg::NextColor),
        event::KeyCode::Left => update(model, ColorCodesMsg::PrevBand),
        event::KeyCode::Right => update(model, ColorCodesMsg::NextBand),
        _ => (),
    }
}

pub fn update(model: &mut ColorCodesToSpecsModel, msg: ColorCodesMsg) {
    match msg {
        ColorCodesMsg::ThreeBands => {
            model.resistor = Resistor::ThreeBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
            };
            model.selected_band = model.selected_band.min(2)
        }
        ColorCodesMsg::FourBands => {
            model.resistor = Resistor::FourBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Brown,
            };
            model.selected_band = model.selected_band.min(3)
        }
        ColorCodesMsg::FiveBands => {
            model.resistor = Resistor::FiveBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
            };
            model.selected_band = model.selected_band.min(4)
        }
        ColorCodesMsg::SixBands => {
            model.resistor = Resistor::SixBand {
                band1: rusistor::Color::Brown,
                band2: rusistor::Color::Black,
                band3: rusistor::Color::Black,
                band4: rusistor::Color::Black,
                band5: rusistor::Color::Brown,
                band6: rusistor::Color::Black,
            };
            model.selected_band = model.selected_band.min(5)
        }
        ColorCodesMsg::NextBand => {
            model.selected_band = (model.selected_band + 1) % model.resistor.bands().len()
        }
        ColorCodesMsg::PrevBand => {
            let bands_count = model.resistor.bands().len();
            model.selected_band = (model.selected_band + (bands_count - 1)) % bands_count
        }
        ColorCodesMsg::NextColor => {
            let current_idx: usize = *model.resistor.bands()[model.selected_band] as usize;
            let mut i: usize = 0;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i += 1;
                let next_color = index_to_color((current_idx + i) % 13);
                resistor = model.resistor.with_color(next_color, model.selected_band);
            }
            model.resistor = resistor.unwrap();
        }
        ColorCodesMsg::PrevColor => {
            let current_idx = *model.resistor.bands()[model.selected_band] as usize;
            let mut i: usize = 13;
            let mut resistor = Err("".to_string());
            while resistor.is_err() {
                i -= 1;
                let next_color = index_to_color((current_idx + i) % 13);
                resistor = model.resistor.with_color(next_color, model.selected_band);
            }
            model.resistor = resistor.unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ColorCodesMsg, update};
    use tusistor_core::model::ColorCodesToSpecsModel;

    #[test]
    fn test_nbands_msg() {
        let mut model = ColorCodesToSpecsModel::default();
        assert_eq!(model.resistor.bands().len(), 6);

        update(&mut model, ColorCodesMsg::ThreeBands);
        assert_eq!(model.resistor.bands().len(), 3);

        update(&mut model, ColorCodesMsg::FourBands);
        assert_eq!(model.resistor.bands().len(), 4);

        update(&mut model, ColorCodesMsg::FiveBands);
        assert_eq!(model.resistor.bands().len(), 5);

        update(&mut model, ColorCodesMsg::SixBands);
        assert_eq!(model.resistor.bands().len(), 6);
    }
}
