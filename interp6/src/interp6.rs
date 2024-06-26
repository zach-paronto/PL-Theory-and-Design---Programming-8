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
            '<' => {
                //using take while, find the number of repeated + signs
                /*
                let count = bytes[i..].iter().take_while(|&&x| x == b'<' as u8).count();
                prog.push(Ops::Left(count as usize));
                i = i + count-1;
                */
                let mut count = 1;
                while i + 1 < bytes.len() && bytes[i + 1] as char == '<' {
                    count += 1;
                    i += 1;
                }
                prog.push(Ops::Left(count));
            },
            '>' => {
                //using take while, find the number of repeated + signs
                let mut count = 1;
                while i + 1 < bytes.len() && bytes[i + 1] as char == '>' {
                    count += 1;
                    i += 1;
                }
                prog.push(Ops::Right(count));
                /*
                let count = bytes[i..].iter().take_while(|&&x| x == b'>' as u8).count();
                prog.push(Ops::Right(count as usize));
                i = i + count-1;
                */
            },
            '+' => {
                //using take while, find the number of repeated + signs
                let mut count = 1;
                while i + 1 < bytes.len() && bytes[i + 1] as char == '+' {
                    count += 1;
                    i += 1;
                }
                prog.push(Ops::Add(count));
                /*
                let count = bytes[i..].iter().take_while(|&&x| x == b'+' as u8).count();
                prog.push(Ops::Add(count as u8));
                i = i + count-1;
                */
            },
            '-' => {
                //using take while, find the number of repeated + signs
                let mut count = 1;
                while i + 1 < bytes.len() && bytes[i + 1] as char == '-' {
                    count += 1;
                    i += 1;
                }
                prog.push(Ops::Sub(count));
                /*
                let count = bytes[i..].iter().take_while(|&&x| x == b'-' as u8).count();
                prog.push(Ops::Sub(count as u8));
                i = i + count-1;
                */
            },
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
            Ops::Left(count) => cc-=count,
            Ops::Right(count) => cc+=count,
            Ops::Add(count) => cells[cc]+=count,
            Ops::Sub(count) => cells[cc]-=count,
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
