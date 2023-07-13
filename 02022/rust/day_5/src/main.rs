use std::fs;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error> {
    // gather input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // create the crates and move lists
    let blank = input.iter().position(|x| x.is_empty()).unwrap(); // split position
    let c_list = &input[0..blank - 1]; // og crates list
    let c_list_r: Vec<&String> = c_list.iter().rev().collect(); //juju to help us build the crate stacks
    let m_list = &input[blank + 1..input.len()]; // move list

    // figure out the crate vectors
    let mut crates_9000: Vec<Vec<char>> = Vec::new(); // vec of vecs
    let mut crates_9001: Vec<Vec<char>> = Vec::new(); // vec of vecs
    let mut crates_fr: bool = true; // detect first run

    // map the crates
    for item in c_list_r {
        let item: Vec<char> = item.chars().collect();
        let mut i = 0;
        while i < (item.len() + 3) / 4 {
            let value: char = item[(i * 4) + 1];
            if crates_fr && value != ' ' {
                crates_9000.push(vec![value]); // p1
                crates_9001.push(vec![value]); // p2
            } else if value != ' ' {
                crates_9000[i].push(value); // p1
                crates_9001[i].push(value); // p2
            }
            i += 1;
        }
        crates_fr = false;
    }

    // figure out the moves
    let mut moves: Vec<(usize, usize, usize)> = Vec::new(); // vec of tuples
    for m in m_list {
        let mut m: Vec<&str> = m.split(' ').collect();
        m.remove(0);
        m.remove(1);
        m.remove(2);
        moves.push((
            m[0].parse().unwrap(),
            m[1].parse().unwrap(),
            m[2].parse().unwrap(),
        ));
    }

    // process the crates
    for mv in moves {

        // part one juju  - grab the val, push it to the new stack, pop it off the old one
        let mut i = 0;
        while i < mv.0 {
            let val: char = *crates_9000[mv.1 - 1].last().unwrap();
            crates_9000[mv.2 - 1].push(val);
            crates_9000[mv.1 - 1].pop();
            i += 1;
        }

        // part two juju - get the nth item from the stack, push it and remove it
        let mut x = mv.0;
        while x > 0 {
            let val: char = crates_9001[mv.1 - 1][crates_9001[mv.1 - 1].len() - x];
            let index:usize = crates_9001[mv.1 - 1].len() - x;
            crates_9001[mv.2 - 1].push(val);
            crates_9001[mv.1 - 1].remove(index);
            x -= 1;
        }
    }

    // get the output messages
    // part one
    let mut crates_9000_msg: Vec<char> = Vec::new();
    for stack in crates_9000 {
        crates_9000_msg.push(*stack.last().unwrap());
    }
    let crates_9000_msg: String = crates_9000_msg.iter().collect();

    // part two
    let mut crates_9001_msg: Vec<char> = Vec::new();
    for stack in crates_9001 {
        crates_9001_msg.push(*stack.last().unwrap());
    }
    let crates_9001_msg: String = crates_9001_msg.iter().collect();

    // output
    println!("Part One - CrateMover 9000 - Crate order: {}", crates_9000_msg);
    println!("Part Two - CrateMover 9001 - Crate order: {}", crates_9001_msg);

    Ok(())
}
