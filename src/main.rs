use std::process;

use dice;
use dice::params;

fn main() {
    let params = params::Parameters::new();

    if let Err(e) = dice::run(params) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
