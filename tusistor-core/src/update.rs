use rusistor::{self};

pub enum ColorCodesMsg {
    ThreeBands,
    FourBands,
    FiveBands,
    SixBands,
    NextBand,
    PrevBand,
    NextColor,
    PrevColor,
}

// todo move to rusistor
pub fn index_to_color(idx: usize) -> rusistor::Color {
    match idx {
        0 => rusistor::Color::Black,
        1 => rusistor::Color::Brown,
        2 => rusistor::Color::Red,
        3 => rusistor::Color::Orange,
        4 => rusistor::Color::Yellow,
        5 => rusistor::Color::Green,
        6 => rusistor::Color::Blue,
        7 => rusistor::Color::Violet,
        8 => rusistor::Color::Grey,
        9 => rusistor::Color::White,
        10 => rusistor::Color::Gold,
        11 => rusistor::Color::Silver,
        12 => rusistor::Color::Pink,
        _ => panic!("unknown color"),
    }
}
