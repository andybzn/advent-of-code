fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input);
    dbg!(answer);
}

fn part_1(input: &str) -> String {
    let mut totals: Vec<u32> = Vec::new();
    let race: Vec<&str> = input.split_whitespace().collect();
    let split: usize = race.iter().position(|&x| x == "Distance:").unwrap();
    let (times, distances) = race.split_at(split);
    let mut race_details: Vec<(u32, u32)> = Vec::new();

    for i in 1..times.len() {
        race_details.push((times[i].parse::<u32>().unwrap(), distances[i].parse::<u32>().unwrap()));
    }

    for race in race_details {
        let mut ways: u32 = 0;
        for millisecond in 0..race.0 {
            let distance: u32 = millisecond * (race.0 - millisecond);
            if distance > race.1 {
                ways += 1;
            }
        }
        totals.push(ways);
    }


    let sum: u32 = totals.iter().product();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input() {
        let answer = part_1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(answer, "288".to_string())
    }
}
