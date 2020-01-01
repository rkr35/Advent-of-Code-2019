#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::io::BufRead;

fn main() {
    let mut input = utilities::get_input_file();

    let mut current_line = String::new();
    let mut fuel_requirement = 0;
    let fuel = |mass| mass / 3 - 2;

    while input
        .read_line(&mut current_line)
        .expect("unable to read line from input file") != 0
    {
        let mass: i32 = current_line
            .trim_end()
            .parse()
            .expect("unable to parse mass");

        current_line.clear();

        let mut f = fuel(mass);

        while f > 0 {
            fuel_requirement += f;
            f = fuel(f);
        }
    }

    println!("{}", fuel_requirement);
}
