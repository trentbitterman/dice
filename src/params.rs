//! # Params
//!
//! Defines helper struct use to parse command
//! line arguments.
//!
//! # Note
//!
//! This module is specific to the `dice` command line
//! utility and should not be used anywhere else.

use clap::{App, Arg};
use std::num;

/// # Parameters
///
/// Generates and holds command line arguments and
/// flags used in the `dice` command line utility.
#[derive(Debug, PartialEq)]
pub struct Parameters {
    number_of_dice: u32,
    number_of_sides: num::NonZeroU32,
    glyphs: bool,
}

impl Parameters {
    /// Creates a new Parameters struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use dice::params;
    ///
    /// let params = params::Parameters::new();
    /// ```
    pub fn new() -> Parameters {
        let matches = Parameters::generate_argument_matcher();

        let number_of_dice = matches.value_of("number").unwrap_or("1");
        let number_of_sides = matches.value_of("sides").unwrap_or("6");
        let glyphs = matches.is_present("glyphs");

        let number_of_dice: u32 = number_of_dice.parse().unwrap_or(1);
        let number_of_sides: u32 = number_of_sides.parse().unwrap_or(6);

        let number_of_sides = if let Some(n) = num::NonZeroU32::new(number_of_sides) {
            n
        } else {
            num::NonZeroU32::new(6).unwrap()
        };

        Parameters {
            number_of_dice,
            number_of_sides,
            glyphs,
        }
    }

    /// Returns the Number of Dice argument.
    pub fn number_of_dice(&self) -> u32 {
        self.number_of_dice
    }

    /// Returns the Number of Sides argument.
    pub fn number_of_sides(&self) -> num::NonZeroU32 {
        self.number_of_sides
    }

    /// Returns if the glyph flag was specified.
    pub fn glyphs(&self) -> bool {
        self.glyphs
    }

    /// Creates an argument matcher using `clap`.
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
            .arg(Arg::with_name("glyphs").short("g").long("glyphs").help(
                "Output roll results with die glyphs when using dice with six or less sides.",
            ))
            .get_matches()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_args() {
        let default = Parameters {
            number_of_dice: 1,
            number_of_sides: num::NonZeroU32::new(6).unwrap(),
            glyphs: false,
        };

        assert_eq!(default, Parameters::new());
    }

    #[test]
    fn get_num_of_dice() {
        assert_eq!(
            5,
            Parameters {
                number_of_dice: 5,
                number_of_sides: num::NonZeroU32::new(3).unwrap(),
                glyphs: false,
            }
            .number_of_dice()
        );
    }

    #[test]
    fn get_num_of_sides() {
        assert_eq!(
            num::NonZeroU32::new(21).unwrap(),
            Parameters {
                number_of_dice: 3,
                number_of_sides: num::NonZeroU32::new(21).unwrap(),
                glyphs: true,
            }
            .number_of_sides()
        );
    }

    #[test]
    fn get_glyphs() {
        assert_eq!(
            true,
            Parameters {
                number_of_dice: 3,
                number_of_sides: num::NonZeroU32::new(21).unwrap(),
                glyphs: true,
            }
            .glyphs()
        );
    }
}
