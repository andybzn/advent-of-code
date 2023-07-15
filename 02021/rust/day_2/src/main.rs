use std::fs;
use std::io::BufRead;

fn main() {
    let input: Vec<String> = fs::read("../../inputs/day_2.txt")
        .expect("Couldn't read file")
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let mut new_depth: i32 = 0;

    for d in input {
        let mut d = d.split_whitespace();
        let dir: &str = d.next().unwrap();
        let amt: i32 = d.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => { horizontal += amt; new_depth += amt * aim },
            "down" => { depth += amt; aim += amt },
            "up" => { depth -= amt; aim -= amt },
            _ => ()
        }
    }

    println!("Part 1: {}", depth * horizontal);
    println!("Part 2: {}", new_depth * horizontal);
}