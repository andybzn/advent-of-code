fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

fn part_2(input: &str) -> String {
    let mut cards: Vec<Vec<&str>> = Vec::new();
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut card: Vec<&str> = line.split_ascii_whitespace().collect();
        card.remove(0);
        card.remove(0);
        cards.push(card);
    }
    for card in 0..=cards.len()-1 {
        total += process_card(card, &cards);
    }
    format!("{}", total)
}

fn process_card(card: usize, deck: &[Vec<&str>]) -> u32 {
    let pipe: usize = deck[card].iter().position(|&x| x == "|").unwrap();
    let (results, to_match) = deck[card].split_at(pipe);
    let mut round_count: usize = 0;
    let mut total: u32 = 1;
    for num in to_match {
        if results.contains(num) {
            round_count += 1;
            total += process_card(card+round_count, deck);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(answer, "30".to_string())
    }
}
