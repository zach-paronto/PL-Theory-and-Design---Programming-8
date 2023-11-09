use std::{
    env, error, fs,
    io::{self, Read, Write},
    collections::HashMap,
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
    //use HashMap collection to make the map
    let mut bmap: HashMap<usize, usize> = HashMap::new();// Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets
    let mut iter = 0;
    while iter < prog.len() {
        match prog[iter] {
            Ops::LBrack => {
                //if the token is an open bracket, add the position to the stack
                bstack.push(iter);
            },
            Ops::RBrack => {
                //if token is close bracket:
                let popped = bstack.pop().unwrap();
                bmap.insert(iter, popped);
                bmap.insert(popped, iter);

            },
            _ => (),
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
                pc = *bmap.get(&pc).unwrap()
            }
            Ops::RBrack if cells[cc] != 0 => {
                //if the token is a close bracket, jump to the corresponding mapped position
                pc = *bmap.get(&pc).unwrap()
            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
