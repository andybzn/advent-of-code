use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    println!("--- Day 8: Haunted Wasteland ---");
    let input: &str = include_str!("../input.txt");
    let (map, instructions) = parse_input(input);
    let answer: String = part_2(map, instructions);
    println!("Part 2: {answer}");
}

fn parse_input(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
    let (instructions, map) = input.split_once('\n').unwrap();
    (
        map.trim()
            .lines()
            .map(|line| {
                let split: Vec<&str> = line.split(' ').collect();
                (split[0], (&split[2][1..4], &split[3][0..3]))
            })
            .collect(),
        instructions.chars().collect(),
    )
}

fn part_2(map: HashMap<&str, (&str, &str)>, instructions: Vec<char>) -> String {
    let positions: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect();

    let mut steps: Vec<u32> = Vec::new();
    for mut position in positions {
        let mut step_counter: u32 = 0;
        while !position.ends_with('Z') {
            for instruction in &instructions {
                position = process(&position, instruction, &map);
                step_counter += 1;
            }
        }
        steps.push(step_counter)
    }

    let totals: u64 = steps.iter().fold(1, |acc, &s| lcm(acc, s as u64));
    format!("{}", totals)
}

fn process(position: &str, instruction: &char, map: &HashMap<&str, (&str, &str)>) -> String {
    let key: &str = position;
    let (left, right) = map.get_key_value(key).unwrap().1;
    let p: &str = match instruction {
        'L' => left,
        'R' => right,
        _ => "",
    };
    String::from(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = parse_input(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(part_2(input.0, input.1), "6".to_string())
    }
}
