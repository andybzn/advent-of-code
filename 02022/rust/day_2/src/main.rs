use std::fs;
use std::io::{BufRead, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error>{

    // gather input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // scoring
    let scores: HashMap<char, i32> = HashMap::from([
        ('A', 1i32),
        ('B', 2i32),
        ('C', 3i32),
        ('X', 1i32),
        ('Y', 2i32),
        ('Z', 3i32),
    ]);
    let mut p1_total_score: i32 = 0;
    let mut p2_total_score: i32 = 0;

    // move combos
    let win: HashMap<char, char> = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A'),
    ]);
    let lose: HashMap<char, char> = HashMap::from([
        ('A', 'C'),
        ('B', 'A'),
        ('C', 'B'),
    ]);
    let draw: HashMap<char, char> = HashMap::from([
        ('A', 'A'),
        ('B', 'B'),
        ('C', 'C'),
    ]);

    // how about a nice game of [rock, paper, scissors]?
    for round in input {
        let plays: Vec<char> = round.chars().collect();
        let play: char = plays[0];
        let code: char = plays[2];

        // Part One
        if  //draw
            (play == 'A' && code == 'X') ||
            (play == 'B' && code == 'Y') ||
            (play == 'C' && code == 'Z') {
                p1_total_score += scores.get(&code).unwrap() + 3;
            }
        else if // win
            (play == 'A' && code == 'Y') ||
            (play == 'B' && code == 'Z') ||
            (play == 'C' && code == 'X') {
                p1_total_score += scores.get(&code).unwrap() + 6;
        } else {
            p1_total_score += scores.get(&code).unwrap(); // lose
        }

        // Part Two
        if code == 'X' {
            p2_total_score += scores.get(lose.get(&play).unwrap()).unwrap(); // lose
        } else if code == 'Y' {
            p2_total_score += scores.get(draw.get(&play).unwrap()).unwrap() + 3; // draw
        } else {
            p2_total_score += scores.get(win.get(&play).unwrap()).unwrap() + 6; // win
        }
    }

    // output
    println!("Part One - Total score: {}", p1_total_score);
    println!("Part Two - Total score: {}", p2_total_score);

    Ok(())
}