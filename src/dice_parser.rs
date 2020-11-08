//! # Dice Parser
//!
//! Defines functions used to parse the
//! custom dice representation used
//! by the `dice` utility.

use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use std::error::Error;
use std::io::Read;
use std::num::NonZeroU32;


use rollset::RollSet;

/// DiceParser
///
/// Reads a file containing sets of
/// dice rolls that can be interpreted
/// by the `dice` utility.
#[derive(Debug)]
pub struct DiceParser {
    dice_filename: &str,
    roll_sets: Vec<RollSet>,
}

impl DiceParser {
    /// Creates a new DiceParser object with the supplied
    /// filename.
    ///
    /// # Examples
    ///
    /// ```
    /// use dice::dice_parser::DiceParser;
    ///
    /// let dice_rolls = DiceParser::new("test/dice_file.txt");
    /// ```
    pub fn new(filename: &str) -> DiceParser {
        DiceParser {dice_filename: filename, roll_sets: Vec::new()}
    }

    pub fn parse_dice(&self) -> Result<(), Box<dyn Error>> {
        let dice_file = File::open(self.dice_filename)?;
        let mut buf_reader = BufReader::new(dice_file);
        let mut file_contents = String::new();
        buf_reader.read_to_string(&mut file_contents)?;

        let re = Regex::new(r"(\d+)d(\d+)").unwrap();

        for cap in re.captures_iter(&file_contents) {
            let num_of_sides = &cap[1];
            let num_of_sides: u32 = num_of_sides.parse()?;
            let number_of_sides = if let Some(n) = NonZeroU32::new(num_of_sides) {
                n
            } else {

            };
        }

        Ok(())
    }
}
