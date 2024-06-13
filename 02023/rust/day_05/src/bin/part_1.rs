use std::collections::HashMap;

fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let parsed: (Vec<u32>, HashMap<String, HashMap<u32, u32>>) = parse_input(input);
    let answer: String = part_1(parsed.0, parsed.1);
    dbg!(answer);
}

fn parse_input(input: &str) -> (Vec<u32>, HashMap<String, HashMap<u32,u32>>) {
    let (s, m) = input.split_once('\n').unwrap();
    let seeds: Vec<u32> = s.strip_prefix("seeds: ").unwrap().split(' ').map(|s| s.parse::<u32>().unwrap()).collect();

    let n = m.strip_prefix('\n').unwrap();
    for line in n.lines() {
        dbg!(&line);
    }

    let maps = HashMap::new();

    dbg!(&seeds);
    dbg!(&n);

    (seeds, maps)
}

fn part_1(seeds: Vec<u32>, maps: HashMap<String, HashMap<u32, u32>>) -> String {
    let mut locations: Vec<u32> = Vec::new();
    for seed in seeds {

    }

    locations.sort();
    format!("{}", locations.first().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let parsed = parse_input(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(part_1(parsed.0, parsed.1), "35".to_string())
    }
}