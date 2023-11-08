use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack(usize),
    RBrack(usize),
    Output,
    Input,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prog = vec![];
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
      _ => todo!("Copy implemenation from interp3, but initialize LBrack and RBrack to have usize::max_value()"),
    }
    }

    // Notice: we drop bmap here, since it isn't needed.
    let mut bstack = vec![];
    let mut i = 0;
    todo!("Copy implementation from interp3, but update the LBrack and RBrack ops directly to store the jump information.");

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            _ => todo!("Copy the implementation from interp3, dropping bmap."),
        }
        pc += 1;
    }
    Ok(())
}
