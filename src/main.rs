extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::distributions::Uniform;
use rand::Rng;

struct Parameters {
    number_of_dice: u32,
    number_of_sides: u32,
}

fn main() {
    let params = get_cl_parameters();

    let rolls = roll_dice(params.number_of_dice, params.number_of_sides);

    output_rolls(&rolls);
}

fn generate_argument_matcher() -> clap::ArgMatches<'static> {
    App::new("dice")
        .version("0.1")
        .author("Trent B. <trentbitterman@comcast.net>")
        .about("Roll any number of dice with any number of sides.")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("N")
                .help("The number of dice to roll.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sides")
                .short("s")
                .long("sides")
                .value_name("N")
                .help("How many sides the dice or die should have.")
                .takes_value(true),
        )
        .get_matches()
}

fn get_cl_parameters() -> Parameters {
    let matches = generate_argument_matcher();

    let number_of_dice = matches.value_of("number").unwrap_or("1");
    let number_of_sides = matches.value_of("sides").unwrap_or("6");

    let number_of_dice: u32 = number_of_dice
        .parse()
        .expect("The number of dice must be a positive, non-zero number");

    let number_of_sides: u32 = number_of_sides
        .parse()
        .expect("The number of sides must be a positive, non-zero number");

    Parameters {
        number_of_dice,
        number_of_sides,
    }
}

fn roll_dice(number_of_dice: u32, number_of_sides: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let range_of_dice = Uniform::new_inclusive(1, number_of_sides);

    let mut die_roll = rng.sample_iter(&range_of_dice);

    let mut rolls: Vec<u32> = Vec::new();

    for _ in 0..number_of_dice {
        rolls.push(die_roll.next().unwrap());
    }

    rolls
}

fn output_rolls(rolls: &Vec<u32>) {
    for roll in rolls.iter() {
        print!("{} ", roll);
    }
    println!("");
}
