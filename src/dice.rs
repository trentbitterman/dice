use rand::{distributions::Uniform, Rng};
use std::fmt;

pub struct RollSet {
    number_of_dice: u32,
    number_of_sides: u32,
    output_glyphs: bool,
    results: Vec<u32>,
}

impl RollSet {
    pub fn new(number_of_dice: u32, number_of_sides: u32, output_glyphs: bool) -> RollSet {
        RollSet {
            number_of_dice,
            number_of_sides,
            output_glyphs: if number_of_sides <= 6 {
                output_glyphs
            } else {
                false
            },
            results: Vec::with_capacity(number_of_dice as usize),
        }
    }

    pub fn roll_dice(&mut self) {
        let range_of_dice = Uniform::new_inclusive(1, self.number_of_sides);

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
