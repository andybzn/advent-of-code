use regex::Regex;

fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input, 12, 14, 13);
    dbg!(answer);
}

fn part_1(input: &str, red_limit: u32, blue_limit: u32, green_limit: u32) -> String {
    let mut valid_games: Vec<u32> = Vec::new();

    // regex patterns
    let re_game_id: Regex = Regex::new(r"^Game (\d+)").unwrap();
    let re_red: Regex = Regex::new(r"(\d+) red").unwrap();
    let re_blue: Regex = Regex::new(r"(\d+) blue").unwrap();
    let re_green: Regex = Regex::new(r"(\d+) green").unwrap();

    for line in input.lines() {
        // get game id
        let id: Vec<u32> = re_game_id
            .find_iter(line)
            .map(|h| {
                h.as_str()
                    .strip_prefix("Game ")
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();

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
        if red_max <= red_limit
            && blue_max <= blue_limit
            && green_max <= green_limit
        {
            valid_games.push(id[0])
        }
    }
    let sum: u32 = valid_games.iter().sum();
    format!("{sum}")
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input() {
        let answer = part_1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            12,
            14,
            13,
        );
        assert_eq!(answer, "8".to_string())
    }
}
