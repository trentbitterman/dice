use clap::{App, Arg};

pub struct Parameters {
    number_of_dice: u32,
    number_of_sides: u32,
    glyphs: bool,
}

impl Parameters {
    pub fn new(number_of_dice: u32, number_of_sides: u32, glyphs: bool) -> Parameters {
        Parameters {
            number_of_dice,
            number_of_sides,
            glyphs,
        }
    }

    pub fn number_of_dice(&self) -> u32 {
        self.number_of_dice
    }

    pub fn number_of_sides(&self) -> u32 {
        self.number_of_sides
    }

    pub fn glyphs(&self) -> bool {
        self.glyphs
    }
}

pub fn get_cl_parameters() -> Parameters {
    let matches = generate_argument_matcher();

    let number_of_dice = matches.value_of("number").unwrap_or("1");
    let number_of_sides = matches.value_of("sides").unwrap_or("6");
    let glyphs = matches.is_present("glyphs");

    let number_of_dice: u32 = number_of_dice.parse().unwrap();
    let number_of_sides: u32 = number_of_sides.parse().unwrap();

    Parameters::new(number_of_dice, number_of_sides, glyphs)
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
