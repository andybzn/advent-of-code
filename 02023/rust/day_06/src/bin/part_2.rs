fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

fn part_2(input: &str) -> String {
    let race: Vec<&str> = input.split_whitespace().collect();
    let split: usize = race.iter().position(|&x| x == "Distance:").unwrap();
    let (times,  distances) = race.split_at(split);
    let race_details: (u64, u64) = (
        times[1..].join("").parse::<u64>().unwrap(),
        distances[1..].join("").parse::<u64>().unwrap()
    );

    let mut ways: u32 = 0;
    for millisecond in 0..race_details.0 {
        let distance: u64 = millisecond * (race_details.0 - millisecond);
        if distance > race_details.1 {
            ways += 1;
        }
    }

    format!("{}", ways)
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(answer, "71503".to_string())
    }
}
