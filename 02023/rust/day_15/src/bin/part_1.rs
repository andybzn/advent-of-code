fn main() {
    let input: &str = include_str!("../input.txt");
    let parsed: Vec<&str> = parse_input(input);
    let answer: String = part_1(parsed);
    dbg!(&answer);
    println!("--- Day 15: Lens Library ---");
    println!("Part 1: {answer}");
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(',').collect()
}

fn part_1(input: Vec<&str>) -> String {
    let mut totals: Vec<u32> = Vec::new();
    for instruction in input {
        let mut total: u32 = 0;
        for char in instruction.chars() {
            total += char as u32;
            total = total * 17;
            total = total % 256;
        }
        totals.push(total);
    }
    let sum: u32 = totals.iter().sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1};
    #[test]
    fn sample_hash() {
        let input = parse_input(
            "HASH",
        );
        assert_eq!(part_1(input), "52".to_string())
    }

    #[test]
    fn sample_instruction() {
        let input = parse_input(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7",
        );
        assert_eq!(part_1(input), "1320".to_string())
    }

    #[test]
    fn actual_input() {
        let input = parse_input(include_str!("../input.txt"));
        assert_eq!(part_1(input), "511343".to_string())
    }
}