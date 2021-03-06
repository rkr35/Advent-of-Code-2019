#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::io::Read;

type Int = usize;

fn main() {
    let mut program = String::new();
    
    utilities::get_input_file()
        .read_to_string(&mut program)
        .expect("unable to read input file into string");
    
    let program: Vec<Int> = program
        .split(',')
        .map(|p| p
            .trim_end()
            .parse()
            .expect("unable to parse data as an integer")
        )
        .collect();

    println!("part1 = {:?}", run(12, 2, program.clone()));

    let [noun, verb] = part2(&program).expect("unable to find noun-verb");
    println!("part2: [noun, verb, 100 * noun + verb] = [{}, {}, {}]",
        noun, verb, 100 * noun + verb);
}
    
fn run(noun: Int, verb: Int, mut program: Vec<Int>) -> Option<Int> {
    *program.get_mut(1).expect("position 1 doesn't exist") = noun;
    *program.get_mut(2).expect("position 2 doesn't exist") = verb;
    
    for i in (0..program.len()).step_by(4) {
        const ADD: Int = 1;
        const MULTIPLY: Int = 2;
        const STOP: Int = 99;
        
        let opcode = *program.get(i).expect("expected opcode");
        
        let [left, right, store] = {
            let o = |i| *program.get(i).expect("missing operand");
            let p = |i| *program.get(i).expect("position doesn't exist");
            let op = |i| o(p(i));
            [op(i+1), op(i+2), p(i+3)]
        };

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

    program.get(0).copied()
}

fn part2(program: &[Int]) -> Option<[Int; 2]> {
    const MAX_CELL_VALUE: Int = 100;
    const TARGET_OUTPUT: Int = 19690720;
    (0..MAX_CELL_VALUE)
        .flat_map(|noun| (0..MAX_CELL_VALUE).map(move |verb| [noun, verb]))
        .find(|[noun, verb]| run(*noun, *verb, program.to_vec())
            .map_or(false, |output| output == TARGET_OUTPUT)
        )
}