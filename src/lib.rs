use std::error::Error;

pub mod num;
pub mod params;
pub mod rollset;

pub fn run(params: params::Parameters) -> Result<(), Box<dyn Error>> {
    let mut rolls = rollset::RollSet::new(
        params.number_of_dice(),
        num::NonZeroPosInteger::new(params.number_of_sides()),
        params.glyphs(),
    );

    rolls.roll_dice();

    println!("{}", rolls);

    Ok(())
}
