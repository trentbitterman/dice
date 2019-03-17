extern crate clap;
extern crate rand;

mod dice;
mod params;

fn main() {
    let params = params::get_cl_parameters();

    let mut rolls = dice::RollSet::new(
        params.number_of_dice(),
        dice::NonZeroPosInteger::new(params.number_of_sides()),
        params.glyphs(),
    );

    rolls.roll_dice();

    println!("{}", rolls);
}
