//! # Dice
//!
//! `dice` is both a command line utility for simulating
//! the rolling of dice using random numbers, it is also
//! a set of modules and functions that help with that
//! aim.

use std::error::Error;

pub mod params;
pub mod rollset;

/// Main logic for the dice command line utility.
pub fn run(params: params::Parameters) -> Result<(), Box<dyn Error>> {
    let mut rolls = rollset::RollSet::new(
        params.number_of_dice(),
        params.number_of_sides(),
        params.glyphs(),
    );

    rolls.roll_dice();

    println!("{}", rolls);

    Ok(())
}
