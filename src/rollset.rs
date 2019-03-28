use rand::{distributions::Uniform, Rng};
use std::{cmp, fmt};

use super::num::NonZeroPosInteger;

#[derive(Debug)]
pub struct RollSet {
    output_glyphs: bool,
    number_of_dice: u32,
    number_of_sides: NonZeroPosInteger,
    results: Vec<u32>,
}

impl RollSet {
    pub fn new(
        number_of_dice: u32,
        number_of_sides: NonZeroPosInteger,
        output_glyphs: bool,
    ) -> RollSet {
        RollSet {
            output_glyphs: if number_of_sides.value() > 6 && output_glyphs {
                panic!(
                    "Glyph output not supported when number_of_sides > 6, number_of_sides is {}.",
                    number_of_sides.value()
                );
            } else {
                output_glyphs
            },
            number_of_dice,
            number_of_sides,
            results: Vec::with_capacity(number_of_dice as usize),
        }
    }

    pub fn roll_dice(&mut self) {
        let range_of_dice = Uniform::new_inclusive(1, self.number_of_sides.value());

        self.results = rand::thread_rng()
            .sample_iter(&range_of_dice)
            .take(self.number_of_dice as usize)
            .collect();
    }

    fn roll_to_glyph(roll: u32) -> &'static str {
        match roll {
            1 => "⚀",
            2 => "⚁",
            3 => "⚂",
            4 => "⚃",
            5 => "⚄",
            6 => "⚅",
            _ => "?",
        }
    }
}

impl fmt::Display for RollSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.output_glyphs {
            write!(
                f,
                "{}",
                self.results
                    .clone()
                    .into_iter()
                    .map(RollSet::roll_to_glyph)
                    .collect::<Vec<&'static str>>()
                    .join(" ")
            )
        } else {
            write!(
                f,
                "{}",
                self.results
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<String>>()
                    .join(" ")
            )
        }
    }
}

impl cmp::PartialEq for RollSet {
    fn eq(&self, other: &RollSet) -> bool {
        self.number_of_dice == other.number_of_dice
            && self.number_of_sides == other.number_of_sides
            && self.output_glyphs == other.output_glyphs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_valid_input() {
        let expected = RollSet {
            output_glyphs: false,
            number_of_sides: NonZeroPosInteger::new(3),
            number_of_dice: 5,
            results: Vec::new(),
        };
        assert_eq!(expected, RollSet::new(5, NonZeroPosInteger::new(3), false));
    }

    #[test]
    #[should_panic(expected = "Glyph output not supported when number_of_sides > 6")]
    fn new_with_invalid_input() {
        RollSet::new(12, NonZeroPosInteger::new(8), true);
    }

    #[test]
    fn roll_seven_dice() {
        let mut dice = RollSet::new(7, NonZeroPosInteger::new(6), false);
        dice.roll_dice();
        assert_eq!(dice.results.len(), 7);
    }

    #[test]
    fn roll_to_glyph_one() {
        assert_eq!("⚀", RollSet::roll_to_glyph(1));
    }

    #[test]
    fn roll_to_glyph_two() {
        assert_eq!("⚁", RollSet::roll_to_glyph(2));
    }

    #[test]
    fn roll_to_glyph_three() {
        assert_eq!("⚂", RollSet::roll_to_glyph(3));
    }

    #[test]
    fn roll_to_glyph_four() {
        assert_eq!("⚃", RollSet::roll_to_glyph(4));
    }

    #[test]
    fn roll_to_glyph_five() {
        assert_eq!("⚄", RollSet::roll_to_glyph(5));
    }

    #[test]
    fn roll_to_glyph_six() {
        assert_eq!("⚅", RollSet::roll_to_glyph(6));
    }

    #[test]
    fn roll_to_glyph_other() {
        assert_eq!("?", RollSet::roll_to_glyph(99));
    }
}
