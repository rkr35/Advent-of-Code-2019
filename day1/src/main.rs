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

    while f.read_line(&mut current_line).expect("unable to read line from input file") != 0 {
        let mass: u32 = current_line.trim_end().parse().expect("unable to parse mass");
        let fuel = mass / 3 - 2;
        fuel_requirement += fuel;
        current_line.clear();
    }

    println!("{}", fuel_requirement);
}
