use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    let mut bmap = vec![]; // Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets
    todo!("Build the bracket map by preprocessing the program.");

    let mut pc = 0;
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] as char {
      _ => todo!("Copy your source code from interp1. You'll need to modify the `[` and `]` instructions."),
    }
        pc += 1;
    }
    Ok(())
}
