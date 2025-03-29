use rusistor::{Color, Resistor};
#[allow(unused_variables)]
fn main() {
    // Example 1: Create a 4-band resistor
    let resistor = Resistor::try_create(vec![
        Color::Red,    // First band
        Color::Violet, // Second band
        Color::Black,  // Third band (multiplier)
        Color::Gold,   // Fourth band (tolerance)
    ])
    .expect("Valid 4-band resistor");

    // Get the specifications of the resistor
    let specs = resistor.specs();
    println!("Resistor Specifications:");
    println!("Ohms: {}", specs.ohm);
    println!("Tolerance: ±{}%", specs.tolerance * 100.0);
    println!("Min Ohms: {}", specs.min_ohm);
    println!("Max Ohms: {}", specs.max_ohm);

    // Example 2: Determine a resistor from its resistance value
    let auto_resistor = Resistor::determine(
        470.0,      // Resistance in ohms
        Some(0.05), // Tolerance (5%)
        None,       // No temperature coefficient
    )
    .expect("Valid resistor");

    println!("\nAuto-generated Resistor Bands:");
    for (idx, band) in auto_resistor.bands().iter().enumerate() {
        println!("Band {}: {}", idx + 1, band);
    }

    // Example 3: Working with different types of resistors
    let zero_ohm = Resistor::try_create(vec![Color::Black]).expect("Zero ohm resistor");

    let three_band = Resistor::try_create(vec![Color::Blue, Color::Grey, Color::Pink])
        .expect("Three-band resistor");

    let six_band = Resistor::try_create(vec![
        Color::Green,
        Color::Blue,
        Color::Black,
        Color::Black,
        Color::Brown,
        Color::Grey,
    ])
    .expect("Six-band resistor");

    // Demonstrate color manipulation
    let modified_resistor = three_band
        .with_color(Color::Red, 2)
        .expect("Modified resistor");

    println!("\nColor Manipulation Example:");
    println!("Original bands: {:?}", three_band.bands());
    println!("Modified bands: {:?}", modified_resistor.bands());
}
