#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut f = File::open("input.txt")
        .map(BufReader::new)
        .expect("unable to open input file");

    let mut current_line = String::new();
    let mut fuel_requirement = 0;
    let fuel = |mass| mass / 3 - 2;

    while f.read_line(&mut current_line).expect("unable to read line from input file") != 0 {
        let mass: i32 = current_line.trim_end().parse().expect("unable to parse mass");
        current_line.clear();

        let mut f = fuel(mass);

        while f > 0 {
            fuel_requirement += f;
            f = fuel(f);
        }
    }

    println!("{}", fuel_requirement);
}
