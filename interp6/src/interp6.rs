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
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    let bytes = fs::read(env::args().nth(1).unwrap())?;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] as char {
            _ => todo!("Copy interp5, update >, <, - op codes to handle repeats."),
        }
        i += 1;
    }
    todo!("Copy interp5 implemenation of loop caching");

    let mut cells = vec![0u8; 10000];
    let mut cc = 0usize;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            _ => todo!("Copy interp5, but update Ops::Sub, Ops::Left, Ops::Right instruction."),
        }
        pc += 1;
    }
    Ok(())
}
