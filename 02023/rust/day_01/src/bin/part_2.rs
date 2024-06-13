fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

fn part_2(input: &str) -> String {
    let mut totals: Vec<u32> = Vec::new();
    for line in input.lines() {
        let pattern: &[char; 10] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let word_pattern: Vec<&str> = Vec::from([
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]);

        let left: usize = get_left(line, pattern, &word_pattern);
        let right: usize = get_right(line, pattern, &word_pattern);

        totals.push(format!("{}{}", left, right).parse::<u32>().unwrap());
    }
    let sum: u32 = totals.iter().sum();
    format!("{}", sum)
}

fn get_lesser(a: (usize, usize), b: (usize, usize)) -> usize {
    if a.0 < b.0 {
        a.1
    } else {
        b.1
    }
}

fn get_greater(a: (usize, usize), b: (usize, usize)) -> usize {
    if a.0 > b.0 {
        a.1
    } else {
        b.1
    }
}

fn get_left(input: &str, pattern: &[char; 10], w_pattern: &[&str]) -> usize {
    // find chars
    let chars: Vec<char> = input.chars().collect();
    let char_found: Option<usize> = input.find(pattern);
    let char_digit: usize = char_found.unwrap_or(999);
    let char_char: usize = if char_digit == 999 {
        999
    } else {
        chars[char_digit].to_string().parse::<usize>().unwrap()
    };
    let char_tup: (usize, usize) = (char_digit, char_char);

    // find word
    let mut vals: Vec<(usize, usize)> = Vec::new();
    for test in w_pattern.iter().enumerate() {
        if let Some(x) = input.find(test.1) {
            vals.push((x, test.0))
        }
    }
    vals.sort();
    let word_tup: (usize, usize) = if !vals.is_empty() {
        vals[0]
    } else {
        (999, 999)
    };

    get_lesser(char_tup, word_tup)
}

fn get_right(input: &str, pattern: &[char; 10], w_pattern: &[&str]) -> usize {
    // find chars
    let chars: Vec<char> = input.chars().collect();
    let char_found: Option<usize> = input.rfind(pattern);
    let char_digit: usize = char_found.unwrap_or(999);
    let char_tup: (usize, usize) = if char_digit == 999 {
        (0, 999)
    } else {
        (
            char_digit,
            chars[char_digit].to_string().parse::<usize>().unwrap(),
        )
    };

    // find word
    let mut vals: Vec<(usize, usize)> = Vec::new();
    for test in w_pattern.iter().enumerate() {
        if let Some(x) = input.rfind(test.1) {
            vals.push((x, test.0))
        }
    }
    vals.sort();
    let word_tup: (usize, usize) = if !vals.is_empty() {
        vals[vals.len() - 1]
    } else {
        (
            char_digit,
            chars[char_digit].to_string().parse::<usize>().unwrap(),
        )
    };

    get_greater(char_tup, word_tup)
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(answer, "358".to_string())
    }
}
