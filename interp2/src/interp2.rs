use std::{
    env, error, fs,
    io::{self, Read, Write},
};
fn main() -> Result<(), Box<dyn error::Error>> {
    let prog = fs::read(env::args().nth(1).unwrap())?;

    // "b" is for bracket
    //use a vector to create the map
    let mut bmap: Vec<usize> = vec![];// Map from a position in the program to the jump location
    let mut bstack = vec![]; // Used to track nested brackets
    let mut iter = 0;
    while iter < prog.len() {
        match prog[iter] as char {
        '[' => {
            //rewrite to follow formula above
            bmap.push(0);
            bstack.push(iter);

        },
        ']' => {
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
    let mut cells = vec![0u8; 10000];
    let mut cc = 0;
    while pc < prog.len() {
        match prog[pc] as char {
            '<' => cc-=1,
            '>' => cc+=1,
            '+' => cells[cc]+=1,
            '-' => cells[cc]-=1,
            '[' if cells[cc] == 0 => {
                //if the token is an open bracket, jump to the corresponding mapped position
                pc = bmap[pc];
            }
            ']' if cells[cc] != 0 => {
                //if the token is a close bracket, jump to the corresponding mapped position
                pc = bmap[pc];
            }
            '.' => io::stdout().write_all(&cells[cc..cc + 1])?,
            ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
            _ => (), /* Ignore any other characters */
        }
        pc += 1;
    }
    Ok(())
}
