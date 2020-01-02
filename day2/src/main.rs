#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::io::Read;

fn main() {
    let mut program = String::new();
    
    utilities::get_input_file()
        .read_to_string(&mut program)
        .expect("unable to read input file into string");
    
    let mut program: Vec<usize> = program
        .split(',')
        .map(|p| p
            .trim_end()
            .parse()
            .expect("unable to parse data as an integer")
        )
        .collect();

    // "...before running the program, replace position 1 with the value 12 and 
    // replace position 2 with the value 2."
    *program.get_mut(1).expect("position 1 doesn't exist") = 12;
    *program.get_mut(2).expect("position 2 doesn't exist") = 2;

    for i in (0..program.len()).step_by(4) {
        const ADD: usize = 1;
        const MULTIPLY: usize = 2;
        const STOP: usize = 99;

        let opcode = *program.get(i).expect("expected opcode");
        
        let left = program.get(i+1).expect("expected left operand");
        let left = *program.get(*left).expect("left position doesn't exist");

        let right = program.get(i+2).expect("expected left operand");
        let right = *program.get(*right).expect("right position doesn't exist");

        let store = *program.get(i+3).expect("expected store operand");
        let store = program
            .get_mut(store)
            .expect("store position doesn't exist");

        match opcode {
            ADD => *store = left + right,
            MULTIPLY => *store = left * right,
            STOP => break,
            unrecognized => panic!("unrecognized opcode: \"{}\"", unrecognized),
        };
    }


    println!("{:?}", program.get(0));
}
