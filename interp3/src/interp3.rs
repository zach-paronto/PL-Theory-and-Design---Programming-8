use std::{
    env, error, fs,
    io::{self, Read, Write},
};
enum Ops {
    Left,
    Right,
    Add,
    Sub,
    LBrack,
    RBrack,
    Output,
    Input,
}
fn main() -> Result<(), Box<dyn error::Error>> {
    /* Notice: prog is now a vec of OpCodes, not a string */
    let mut prog = vec![];

    /* First parse the program into a sequence of opcodes */
    for b in fs::read(env::args().nth(1).unwrap())? {
        match b as char {
            _ => todo!("Translate all of the commands into their opcodes."),
        }
    }

    let mut pc = 0;
    let mut bmap = vec![];
    let mut bstack = vec![];
    todo!("Copy implementation from interp2, but match on opcodes instead of characters.");

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] {
            _ => todo!(
                "Copy implementation from interp2, but match on opcodes instead of characters."
            ),
        }
        pc += 1;
    }
    Ok(())
}
