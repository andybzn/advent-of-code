
use regex::Regex;

fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

fn part_2(input: &str) -> String {
    let mut cube_powers: Vec<u32> = Vec::new();

    // regex patterns
    let re_red: Regex = Regex::new(r"(\d+) red").unwrap();
    let re_blue: Regex = Regex::new(r"(\d+) blue").unwrap();
    let re_green: Regex = Regex::new(r"(\d+) green").unwrap();

    for line in input.lines() {
        // grab colors from each line
        let mut reds: Vec<u32> = re_red
            .find_iter(line)
            .map(|h| {
                h.as_str()
                    .strip_suffix(" red")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();
        let mut blues: Vec<u32> = re_blue
            .find_iter(line)
            .map(|h| {
                h.as_str()
                    .strip_suffix(" blue")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();
        let mut greens: Vec<u32> = re_green
            .find_iter(line)
            .map(|h| {
                h.as_str()
                    .strip_suffix(" green")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

        // sort colors
        reds.sort();
        blues.sort();
        greens.sort();

        // grab the max vals
        let red_max: u32 = *reds.last().unwrap();
        let blue_max: u32 = *blues.last().unwrap();
        let green_max: u32 = *greens.last().unwrap();

        // logic
        let power: u32 = red_max * blue_max * green_max;
        cube_powers.push(power);
    }
    let sum: u32 = cube_powers.iter().sum();
    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(answer, "2286".to_string())
    }
}
