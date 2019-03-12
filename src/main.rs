extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::distributions::Uniform;
use rand::Rng;
use std::fmt::Display;

struct Parameters {
    number_of_dice: u32,
    number_of_sides: u32,
    glyphs: bool,
}

fn main() {
    let params = get_cl_parameters();

    let rolls = roll_dice(params.number_of_dice, params.number_of_sides);

    if params.glyphs && params.number_of_sides <= 6 {
        let glyphs: Vec<char> = rolls.iter().map(roll_to_glyph).collect();
        output_rolls(&glyphs);
    } else {
        output_rolls(&rolls);
    }
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
                .takes_value(true)
                .validator(|v| {
                    if v.parse::<u32>().is_ok() {
                        return Ok(());
                    }
                    Err(String::from(
                        "The value should be an integer greater than or equal to 0.",
                    ))
                }),
        )
        .arg(
            Arg::with_name("sides")
                .short("s")
                .long("sides")
                .value_name("N")
                .help("How many sides the dice or die should have.")
                .takes_value(true)
                .validator(|v| {
                    if v.parse::<u32>().is_ok() && v.parse::<u32>().unwrap() > 0 {
                        return Ok(());
                    }
                    Err(String::from(
                        "The value should be an integer greater than 0.",
                    ))
                }),
        )
        .arg(
            Arg::with_name("glyphs").short("g").long("glyphs").help(
                "Output roll results with die glyphs when using dice with six or less sides.",
            ),
        )
        .get_matches()
}

fn get_cl_parameters() -> Parameters {
    let matches = generate_argument_matcher();

    let number_of_dice = matches.value_of("number").unwrap_or("1");
    let number_of_sides = matches.value_of("sides").unwrap_or("6");
    let glyphs = matches.is_present("glyphs");

    let number_of_dice: u32 = number_of_dice.parse().unwrap();
    let number_of_sides: u32 = number_of_sides.parse().unwrap();

    Parameters {
        number_of_dice,
        number_of_sides,
        glyphs,
    }
}

fn roll_dice(number_of_dice: u32, number_of_sides: u32) -> Vec<u32> {
    let range_of_dice = Uniform::new_inclusive(1, number_of_sides);

    let rolls: Vec<u32> = rand::thread_rng()
        .sample_iter(&range_of_dice)
        .take(number_of_dice as usize)
        .collect();

    rolls
}

fn output_rolls<T: Display>(rolls: &Vec<T>) {
    for roll in rolls.iter() {
        print!("{} ", roll);
    }
    println!("");
}

fn roll_to_glyph(roll: &u32) -> char {
    match roll {
        1 => '⚀',
        2 => '⚁',
        3 => '⚂',
        4 => '⚃',
        5 => '⚄',
        6 => '⚅',
        _ => '?',
    }
}
