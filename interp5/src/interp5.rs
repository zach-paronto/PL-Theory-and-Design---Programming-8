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
            '<' => prog.push(Ops::Left),
            '>' => prog.push(Ops::Right),
            //'+' altered to also take a u8 value representing the number of repeated + signs
            '+' => {
                //using take while, find the number of repeated + signs
                let count = bytes[i..].iter().take_while(|&&x| x as char == '+').count();
                prog.push(Ops::Add(count as u8));
                i += count - 1;
            },
            '-' => prog.push(Ops::Sub),
            '[' => prog.push(Ops::LBrack (usize::MAX)),
            ']' => prog.push(Ops::RBrack (usize::MAX)),
            '.' => prog.push(Ops::Output),
            ',' => prog.push(Ops::Input),
            _ => (), 
        }
        i += 1;
    }
    
    // Notice: we drop bmap here, since it isn't needed.
    let mut bstack = vec![];
    i = 0;
    while i < prog.len() {
        match prog[i] {
            Ops::LBrack(_) => {
                //if the token is an open bracket, add the position to the stack
                bstack.push(Ops::LBrack(i));
            },
            Ops::RBrack(_) => {
                let popped = bstack.pop().unwrap();
                if let Ops::LBrack(jump_pos) = popped {
                    prog[i] = Ops::RBrack(jump_pos);
                    prog[jump_pos] = Ops::LBrack(i);
                }
            },
            _ => (),
        }
        i += 1;
    }

    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    let mut pc = 0;
    while pc < prog.len() {
        match prog[pc] {
            Ops::Left => cc-=1,
            Ops::Right => cc+=1,
            //update Add code to add the stored count value to cells[cc]
            Ops::Add(count) => cells[cc]+=count,
            Ops::Sub => cells[cc]-=1,
            Ops::LBrack(_) if cells[cc] == 0 => {
                //if the token is an open bracket, jump to the corresponding position in the opcode
                if let Ops::LBrack(jump_pos) = prog[pc] {
                    pc = jump_pos;
                }
            }
            Ops::RBrack(_) if cells[cc] != 0 => {
                //if the token is a close bracket, jump to the corresponding position in the opcode
                if let Ops::RBrack(jump_pos) = prog[pc] {
                    pc = jump_pos;
                }

            }
            Ops::Output => io::stdout().write_all(&cells[cc..cc + 1])?,
            Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
