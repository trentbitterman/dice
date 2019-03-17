use rand::{distributions::Uniform, Rng};
use std::fmt;

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
                    "Glyph output not supported when number_of_sides > 6. number_of_sides is {}.",
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
                    .map(roll_to_glyph)
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

pub struct NonZeroPosInteger {
    n: u32,
}

impl NonZeroPosInteger {
    pub fn new(n: u32) -> NonZeroPosInteger {
        if n == 0 {
            panic!("Input must be an integer greater than zero.")
        }

        NonZeroPosInteger { n }
    }

    pub fn value(&self) -> u32 {
        self.n
    }
}

pub fn roll_to_glyph(roll: u32) -> &'static str {
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
