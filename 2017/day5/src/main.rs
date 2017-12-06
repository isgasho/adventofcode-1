use std::env;
use std::fs::File;
use std::io::prelude::*;

fn nop() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("bork");
        return;
    }
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut instructions: Vec<i32> = Vec::new();

    for num in contents.split("\n") {
        let digit = num.parse::<i32>();
        match digit {
            Ok(v) => instructions.push(v),
            Err(_) => nop(),
        }
    }

    let max = (instructions.len()-1) as i32;
    let mut pc: i32 = 0;
    let mut steps = 0;


    loop {
        if pc < 0 || pc > max {
            break;
        }

        let current = instructions[pc as usize];

        if current > 2 {
            instructions[pc as usize] -= 1;
        }
        else {
            instructions[pc as usize] += 1;
        }

        pc += current;

        steps += 1;
    }


    println!("{:#?}", steps);
}
