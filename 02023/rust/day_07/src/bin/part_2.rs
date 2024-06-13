use std::collections::HashMap;

fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

fn part_2(input: &str) -> String {
    let mut totals: Vec<u32> = Vec::new();
    let scoring: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 1),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    let mut card_scores: Vec<(&str, u32, u32, u32)> = Vec::new(); // face values, bet, type, face score
    for line in input.lines() {
        let l: Vec<&str> = line.split(' ').collect();
        let (hand, bet) = (l[0], l[1].parse::<u32>().unwrap());
        card_scores.push((hand, bet, get_type(hand), get_score(hand, &scoring)));
    }

    // rank: hand type, then score cards
    card_scores.sort_unstable_by(|a, b| (a.2, a.3).cmp(&(b.2, b.3)));
    for hand in card_scores.iter().enumerate() {
        totals.push(hand.1 .1 * (hand.0 as u32 + 1))
    }

    dbg!(card_scores);

    let sum: u32 = totals.iter().sum();
    format!("{}", sum)
}

fn get_type(hand: &str) -> u32 {
    let hand_cards: Vec<char> = hand.chars().collect();
    let mut counts: Vec<u32> = Vec::new();
    // fuck
    // figure out how to pad the nums
    if hand_cards.contains(&'J') {
        let joker_count: u32 = hand.matches('J').count() as u32;
        for card in hand_cards {
            if card != 'J' {
                let count: u32 = hand.matches(card).count() as u32;
                counts.push(count);
            }
        }
        counts.sort();
        // dbg!(&counts);
        // update the last array elems based on the joker count
        let max = match joker_count {
            5 => 5,
            _ => *counts.last().unwrap()
        };

        for i in 0..counts.len() {
            if counts[i] == max {
                counts[i] += joker_count;
            }
        }
        for _i in 1..=joker_count {
            counts.push(max+joker_count);
        }
        // dbg!(&counts);
    } else {
        for card in hand_cards {
            let count: u32 = hand.matches(card).count() as u32;
            counts.push(count);
        }
        counts.sort();
    }

    dbg!(hand, &counts);
    match counts[..] {
        [1, 1, 1, 1, 1] => 1, // high card
        [1, 1, 1, 2, 2] => 2, // one pair
        [1, 2, 2, 2, 2] => 3, // two pair
        [1, 1, 3, 3, 3] => 4, // three of a kind
        [2, 2, 3, 3, 3] => 5, // full house
        [1, 4, 4, 4, 4] => 6, // four of a kind
        [5, 5, 5, 5, 5] => 7, // five of a kind
        _ => 0,
    }
}

fn get_score(hand: &str, scoring: &HashMap<char, u32>) -> u32 {
    let mut score: Vec<u32> = Vec::new();
    for card in hand.chars().enumerate() {
        let value = *scoring.get_key_value(&card.1).unwrap().1;
        score.push(value);
    }
    score.iter().fold(0, |a, &x| a * if x < 10 { 10 } else { 100 } + x)
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(answer, "5905".to_string())
    }
}

/* wrong answers
254761880
254766567 - too low

254944762 - in the range but wrong

254968697 - too high
255379334
 */