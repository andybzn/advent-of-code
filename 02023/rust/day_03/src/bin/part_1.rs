fn main() {
    println!("Part 1!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_1(input);
    dbg!(answer);
}

fn part_1(input: &str) -> String {
    let mut line_chars: Vec<Vec<char>> = Vec::new();
    let mut part_nums: Vec<u32> = Vec::new();

    // split the lines into chars.
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        line_chars.push(chars);
    }

    // parse rows
    for row in line_chars.iter().enumerate() {
        // get the current line num below
        let line_num: usize = row.0;
        let mut digit_buffer: Vec<char> = Vec::new();
        let mut symbol_adj_buffer: Vec<bool> = Vec::new();

        for char in row.1.iter().enumerate() {
            let char_num: usize = char.0;
            if char.1.is_ascii_digit() {
                digit_buffer.push(*char.1);
                let symbol_adjacent: bool = check_left(&line_chars, line_num, char_num)
                    || check_right(&line_chars, line_num, char_num)
                    || check_above(&line_chars, line_num, char_num, row.1.len())
                    || check_below(&line_chars, line_num, char_num, row.1.len());
                symbol_adj_buffer.push(symbol_adjacent);
            } else if !digit_buffer.is_empty() && symbol_adj_buffer.contains(&true) {
                let num: u32 = digit_buffer
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                part_nums.push(num);
                digit_buffer.clear();
                symbol_adj_buffer.clear();
            } else {
                digit_buffer.clear();
                symbol_adj_buffer.clear();
            }
            if char_num == line_chars[line_num].len() - 1
                && !digit_buffer.is_empty()
                && symbol_adj_buffer.contains(&true)
            {
                let num: u32 = digit_buffer
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                part_nums.push(num);
                digit_buffer.clear();
                symbol_adj_buffer.clear();
            }
        }
    }
    let sum: u32 = part_nums.iter().sum();
    format!("{sum}")
}

fn check_above(data: &[Vec<char>], row: usize, pos: usize, len: usize) -> bool {
    if row == 0 {
        false
    } else {
        !data[row - 1][pos].is_ascii_digit() && data[row - 1][pos] != '.'
            || pos > 0
                && pos <= len
                && !data[row - 1][pos - 1].is_ascii_digit()
                && data[row - 1][pos - 1] != '.'
            || pos < len - 1
                && !data[row - 1][pos + 1].is_ascii_digit()
                && data[row - 1][pos + 1] != '.'
    }
}

fn check_below(data: &[Vec<char>], row: usize, pos: usize, len: usize) -> bool {
    if row == data.len() {
        false
    } else if row == data.len() - 1 {
        !data[row][pos].is_ascii_digit() && data[row][pos] != '.'
            || pos > 0
                && pos < len
                && !data[row][pos - 1].is_ascii_digit()
                && data[row][pos - 1] != '.'
            || pos < len - 1 && !data[row][pos + 1].is_ascii_digit() && data[row][pos + 1] != '.'
    } else {
        !data[row + 1][pos].is_ascii_digit() && data[row + 1][pos] != '.'
            || pos > 0
                && pos < len - 1
                && !data[row + 1][pos - 1].is_ascii_digit()
                && data[row + 1][pos - 1] != '.'
            || pos < len - 1
                && !data[row + 1][pos + 1].is_ascii_digit()
                && data[row + 1][pos + 1] != '.'
    }
}

fn check_left(data: &[Vec<char>], row: usize, pos: usize) -> bool {
    if pos == 0 {
        false
    } else {
        !data[row][pos - 1].is_ascii_digit() && data[row][pos - 1] != '.'
    }
}

fn check_right(data: &[Vec<char>], row: usize, pos: usize) -> bool {
    if pos == data[row].len() {
        false
    } else if pos == data[row].len() - 1 {
        !data[row][pos].is_ascii_digit() && data[row][pos] != '.'
    } else {
        !data[row][pos + 1].is_ascii_digit() && data[row][pos + 1] != '.'
    }
}

#[cfg(test)]
mod tests {
    use crate::part_1;
    #[test]
    fn sample_input() {
        let answer = part_1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
..........
.......980
......*...",
        );
        // assert_eq!(answer, "4361".to_string())
        assert_eq!(answer, "5341".to_string())
    }
}
