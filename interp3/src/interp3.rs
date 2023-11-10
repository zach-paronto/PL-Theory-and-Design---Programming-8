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
            //translating chars into their Ops opcodes
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            '+' => prog.push(Ops::Add),
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack),
            ']' => prog.push(Ops::RBrack),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (), 
            
        }
    }

    // "b" is for bracket
    //use a vector to create the map
    let mut bmap: Vec<usize> = vec![];// Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets
    let mut iter = 0;
    while iter < prog.len() {
        match prog[iter] {
            Ops::LBrack => {
                //rewrite to follow formula above
                bmap.push(0);
                bstack.push(iter);

            },
            Ops::RBrack => {
                let popped = bstack.pop().unwrap();
                bmap.push(0);
                bmap[iter] = popped;
                bmap[popped] = iter;
            },
            _ => {
                bmap.push(0);
            },
        }
        iter += 1;
    }

    let mut pc = 0;
    let mut cells = vec![0u8; 1000000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left => cc-=1,
            Ops::Right => cc+=1,
            Ops::Add => cells[cc]+=1,
            Ops::Sub => cells[cc]-=1,
            Ops::LBrack if cells[cc] == 0 => {
                //if the token is an open bracket, jump to the corresponding mapped position
                pc = bmap[pc];
            }
            Ops::RBrack if cells[cc] != 0 => {
                //if the token is a close bracket, jump to the corresponding mapped position
                pc = bmap[pc];
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
