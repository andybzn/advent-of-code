fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input);
    dbg!(answer);
}

fn part_1(input: &str) -> String {
    let mut totals: Vec<u32> = Vec::new();
    for line in input.lines() {
        let pattern: &[char; 10] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let chars: Vec<char> = line.chars().collect();
        let left: char = chars[line.find(pattern).unwrap()];
        let right: char = chars[line.rfind(pattern).unwrap()];
        totals.push(format!("{}{}", left, right).parse::<u32>().unwrap());
    }
    let sum: u32 = totals.iter().sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input() {
        let answer = part_1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(answer, "142".to_string())
    }
}
