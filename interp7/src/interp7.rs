use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left(usize),
    Right(usize),
    Add(u8),
    Sub(u8),
    LBrack(usize),
    RBrack(usize),
    Zero,
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;

    // Compile
    while i < bytes.len() {
        match bytes[i] as char {
            _ => todo!("Copy interp6 implementation"),
        }
        i += 1;
    }

    // Optimize
    /* Iterate through the program, in search of our "Zero" optimization */
    todo!("Implement Zero Op Code optimization");

    todo!("Copy loop caching implementation from interp6.");

    // Interpret / Evaluate
    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            _ => todo!("Copy interp6 implementation, add handling for Zero Op Code"),
        }
        pc += 1;
    }
    Ok(())
}
