fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input);
    dbg!(answer);
}

fn part_1(input: &str) -> String {
    let mut totals: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut card: Vec<&str> = line.split_ascii_whitespace().collect();
        card.remove(0);
        card.remove(0);
        let pipe: usize = card.iter().position(|&x| x == "|").unwrap();
        let (results, to_match) = card.split_at(pipe);
        let mut card_total: u32 = 0;

        for num in to_match {
            if results.contains(num) {
                card_total = increment(card_total);
            }
        }
        totals.push(card_total);
    }
    let sum: u32 = totals.iter().sum();
    format!("{}", sum)
}

fn increment(value: u32) -> u32 {
    if value == 0 {
        1
    } else {
        value * 2
    }
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input() {
        let answer = part_1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(answer, "13".to_string())
    }
}
