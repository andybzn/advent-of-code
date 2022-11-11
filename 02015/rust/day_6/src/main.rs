// imports
use std::io::{Error, BufRead};
use std::fs;
use regex::Regex;

fn main() -> Result<(), Error> {

    // hold onto your hats cos this is gonna be weird
    // generate 1000x1000 grid
    let mut grid = Vec::new();
    let i = 0..1000;
    for _val in i {
        let mut new_vec = Vec::new();
        let x = 0..1000;
        for y in x {
            new_vec.push([y, 0, 0]);
        };
        grid.push(new_vec); 
    };

    // parse input file
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // regex
    let instr_on = Regex::new("(turn on)").unwrap();
    let instr_off = Regex::new("(turn off)").unwrap();
    let instr_tog = Regex::new("(toggle)").unwrap();
    let instr_dig = Regex::new(r"(\d{1,3})").unwrap();

    // light logic
    for instruction in input {

        // states
        let mut on: bool = false;
        let mut off: bool = false;
        let mut toggle: bool = false;

        // pull the instruction (regex)
        if instr_on.is_match(&instruction) {
            on = true;
        } else if instr_off.is_match(&instruction) {
            off = true;
        } else if instr_tog.is_match(&instruction) {
            toggle  = true;
        } else {
            println!("help");
        };

        // pull the coordinates
        let matches = instr_dig.captures_iter(&instruction);
        let mut coords: Vec<usize> = Vec::new();
        for coord in matches {
            let val:usize = coord[0].parse().unwrap();
            coords.push(val);
        };
        
        // map the coordinates
        let from_pos = coords[0];
        let from_vec = coords[1];
        let to_pos = coords[2];
        let to_vec = coords[3];

        // process the vectors
        for vec in from_vec..(to_vec + 1) {
            for light in from_pos..(to_pos + 1) {
                if toggle {
                    if grid[vec][light][1] == 0 {
                        grid[vec][light][1] = 1;
                    } else {
                        grid[vec][light][1] = 0;
                    };
                    grid[vec][light][2] += 2;
                } else if on {
                    grid[vec][light][1] = 1;
                    grid[vec][light][2] += 1;
                } else if off {
                    grid[vec][light][1] = 0;
                    if grid[vec][light][2] != 0 {
                        grid[vec][light][2] -= 1;
                    };
                } else {
                    println!("ruh roh");
                };
            };
        };
    };

    // count the remaining lights & brightness
    let mut lit_count: i128 = 0;
    let mut brightness: i128 = 0;
    for x in grid {
        for y in x{
            if y[1] == 1 {
                lit_count += 1;
            };
            brightness += y[2];
        };
    };

    // output
    println!("Part One - Lights lit: {}", lit_count); 
    println!("Part Two - Brightness: {}", brightness); 

    Ok(())
}