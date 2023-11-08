use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add(u8),
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            _ => todo!("Copy interp4 implementation, but update '+'"),
        }
        i += 1;
    }
    todo!("Copy interp4 implemenation of loop caching");

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            _ => todo!("Copy interp4, but update Ops::Add instruction."),
        }
        pc += 1;
    }
    Ok(())
}
