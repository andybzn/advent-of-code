use std::collections::HashMap;

fn main() {
    println!("--- Day 8: Haunted Wasteland ---");
    let input: &str = include_str!("../input.txt");
    let (map, instructions) = parse_input(input);
    let answer: String = part_1(map, instructions);
    println!("Part 1: {answer}");
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

fn part_1(map: HashMap<&str, (&str, &str)>, instructions: Vec<char>) -> String {
    let mut step_counter: u32 = 0;
    let mut position = "AAA";
    while position != "ZZZ" {
        for instruction in &instructions {
            let (left, right) = map.get_key_value(position).unwrap().1;
            position = match instruction {
                'L' => left,
                'R' => right,
                _ => "",
            };
            step_counter += 1;
        }
    }
    format!("{}", step_counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = parse_input(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(part_1(input.0, input.1), "2".to_string())
    }

    #[test]
    fn sample_input_2() {
        let input = parse_input(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(part_1(input.0, input.1), "6".to_string())
    }
}
