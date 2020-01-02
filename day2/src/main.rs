#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

use std::io::Read;

enum Kind {
    Add,
    Multiply,
}

struct Opcode {
    kind: Kind,
    left: usize,
    right: usize,
    store: usize,
}

impl Opcode {
    fn execute(&self, program: &mut [usize]) {
        let left = program[self.left];
        let right  = program[self.right];
        program[self.store] = match self.kind {
            Kind::Add => left + right,
            Kind::Multiply => left * right,
        };
    }
}

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
    *program.get_mut(1).expect("unable to set position 1") = 12;
    *program.get_mut(2).expect("unable to set position 2") = 2;

    for i in (0..program.len()).step_by(4) {
        let opcode = match *program.get(i).expect("expected opcode") {
            1 => Kind::Add,
            2 => Kind::Multiply,
            99 => break,
            unrecognized => panic!("unrecognized opcode: \"{}\"", unrecognized),
        };
        
        let opcode = Opcode {
            kind: opcode,
            left: *program.get(i+1).expect("expected left operand"),
            right: *program.get(i+2).expect("expected left operand"),
            store: *program.get(i+3).expect("expected store operand"),
        };

        opcode.execute(&mut program);
    }


    println!("{:?}", program.get(0));
}
