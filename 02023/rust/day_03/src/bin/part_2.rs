fn main() {
    println!("Part 2!");
    let input: &str = include_str!("../input.txt");
    let answer: String = part_2(input);
    dbg!(answer);
}

#[allow(clippy::too_many_lines)]
fn part_2(input: &str) -> String {
    let mut line_chars: Vec<Vec<char>> = Vec::new();
    let mut part_nums: Vec<u32> = Vec::new();

    // split the lines into chars
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        line_chars.push(chars);
    }

    // second loop
    for row in line_chars.iter().enumerate() {
        // get the current line num below
        let line_num: usize = row.0;
        println!("{:#?}", row.1);

        for char in row.1.iter().enumerate() {
            let char_num: usize = char.0;
            if char.1 == &'*' {
                let mut part_buffer: Vec<u32> = Vec::new();

                let mut counter: usize = 0;
                let mut row_above: bool = false;
                let mut row_below: bool = false;
                let mut left: bool = false;
                let mut right: bool = false;

                // amend these to return a count of digits found
                if check_left(&line_chars, line_num, char_num) {
                    counter += 1;
                    left = true;
                }
                if check_right(&line_chars, line_num, char_num) {
                    counter += 1;
                    right = true;
                }
                match check_above(&line_chars, line_num, char_num) {
                    (true, x) => {
                        counter += x;
                        row_above = true;
                    },
                    (false, _) => (),
                }
                match check_below(&line_chars, line_num, char_num) {
                    (true, x) => {
                       counter += x;
                       row_below = true;
                    },
                    (false, _) => (),
                }

                dbg!(left, right, row_above, row_below, counter);



                if counter == 2 {
                    // get part numbers
                    if left {
                        // get ascii_digits to the left of the current char_num
                        let mut digits: Vec<char> = Vec::new();
                        for i in char_num..=0 {
                            if line_chars[line_num][i].is_ascii_digit() {
                                digits.push(line_chars[line_num][i]);
                            } else {
                                break;
                            }
                        }
                        // reverse the digits, collect as u32 and push
                        digits.reverse();
                        let part: u32 = digits.iter().collect::<String>().parse::<u32>().unwrap();
                        part_buffer.push(part);
                    }
                    if right {
                        // get ascii_digits to the right of the current char_num
                        let mut digits: Vec<char> = Vec::new();
                        for i in char_num..line_chars[line_num].len() {
                            if line_chars[line_num][i].is_ascii_digit() {
                                digits.push(line_chars[line_num][i]);
                            } else {
                                break;
                            }
                        }
                        // collect as u32 and push
                        let part: u32 = digits.iter().collect::<String>().parse::<u32>().unwrap();
                        part_buffer.push(part);
                    }
                    if row_above {
                        let mut digit: Vec<char> = Vec::new();
                        let mut left_buffer: Vec<char> = Vec::new();
                        let mut right_buffer: Vec<char> = Vec::new();

                        if line_chars[line_num - 1][char_num].is_ascii_digit() {
                            digit.push(line_chars[line_num - 1][char_num]);
                        }
                        // left
                        for i in char_num..=0 {
                            if line_chars[line_num - 1][i].is_ascii_digit() {
                                left_buffer.push(line_chars[line_num - 1][i]);
                            } else {
                                break;
                            }
                        }
                        // right
                        for i in char_num..line_chars[line_num - 1].len() {
                            if line_chars[line_num - 1][i].is_ascii_digit() {
                                right_buffer.push(line_chars[line_num - 1][i]);
                            } else {
                                break;
                            }
                        }
                        concat_buffers(left_buffer, &digit, &right_buffer)
                            .iter()
                            .for_each(|x| part_buffer.push(*x));
                    }

                    // row_below
                    if row_below {
                        let mut digit: Vec<char> = Vec::new();
                        let mut left_buffer: Vec<char> = Vec::new();
                        let mut right_buffer: Vec<char> = Vec::new();

                        if line_chars[line_num + 1][char_num].is_ascii_digit() {
                            digit.push(line_chars[line_num + 1][char_num]);
                        }
                        // left
                        for i in char_num..=0 {
                            if line_chars[line_num + 1][i].is_ascii_digit() {
                                left_buffer.push(line_chars[line_num + 1][i]);
                            } else {
                                break;
                            }
                        }
                        // right
                        for i in char_num..line_chars[line_num + 1].len() {
                            if line_chars[line_num + 1][i].is_ascii_digit() {
                                right_buffer.push(line_chars[line_num + 1][i]);
                            } else {
                                break;
                            }
                        }
                        concat_buffers(left_buffer, &digit, &right_buffer)
                            .iter()
                            .for_each(|x| part_buffer.push(*x));
                    }

                    println!("{:#?}", part_buffer);
                    let product: u32 = part_buffer.iter().product();
                    part_nums.push(product);
                }
            }
        }
    }
    dbg!(&part_nums);
    let sum: u32 = part_nums.iter().sum();
    format!("{sum}")
}

fn check_above(data: &[Vec<char>], row: usize, pos: usize) -> (bool, usize) {
    let len = data[row].len();
    let mut found: bool = false;
    let mut count: usize = 0;
    if row == 0 {
        found = false;
        count = 0;
    } else {
        if data[row - 1][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row - 1][pos - 1].is_ascii_digit())
            && (pos < len - 1 && data[row - 1][pos + 1].is_ascii_digit()) {
            count += 1; found = true;
        } else if data[row - 1][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row - 1][pos - 1].is_ascii_digit()) {
            count += 1; found = true;
        } else if data[row - 1][pos].is_ascii_digit()
            && (pos < len - 1 && data[row - 1][pos + 1].is_ascii_digit()) {
            count += 1; found = true;
        } else {
            if data[row - 1][pos].is_ascii_digit() { count += 1; found = true; };
            if pos > 0 && pos <= len && data[row - 1][pos - 1].is_ascii_digit() { count += 1; found = true; };
            if pos < len - 1 && data[row - 1][pos + 1].is_ascii_digit() { count += 1; found = true; };
        }
    }
    (found, count)
}

fn check_below(data: &[Vec<char>], row: usize, pos: usize) -> (bool, usize) {
    let len = data[row].len();
    let mut found: bool = false;
    let mut count: usize = 0;
    if row == data.len() {
        found = false;
        count = 0;
    } else if row == data.len() - 1 {
        if data[row][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row][pos - 1].is_ascii_digit())
            && (pos < len - 1 && data[row][pos + 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else if data[row][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row][pos - 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else if data[row][pos].is_ascii_digit()
            && (pos < len - 1 && data[row][pos + 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else {
            if data[row][pos].is_ascii_digit() {
                count += 1;
                found = true;
            };
            if pos > 0 && pos <= len && data[row][pos - 1].is_ascii_digit() {
                count += 1;
                found = true;
            };
            if pos < len - 1 && data[row][pos + 1].is_ascii_digit() {
                count += 1;
                found = true;
            };
        }
    } else {
        if data[row + 1][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row + 1][pos - 1].is_ascii_digit())
            && (pos < len - 1 && data[row + 1][pos + 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else if data[row + 1][pos].is_ascii_digit()
            && (pos > 0 && pos <= len && data[row + 1][pos - 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else if data[row + 1][pos].is_ascii_digit()
            && (pos < len - 1 && data[row + 1][pos + 1].is_ascii_digit()) {
            count += 1;
            found = true;
        } else {
            if data[row + 1][pos].is_ascii_digit() {
                count += 1;
                found = true;
            };
            if pos > 0 && pos <= len && data[row + 1][pos - 1].is_ascii_digit() {
                count += 1;
                found = true;
            };
            if pos < len - 1 && data[row + 1][pos + 1].is_ascii_digit() {
                count += 1;
                found = true;
            };
        }
    }
    (found, count)
}

fn check_left(data: &[Vec<char>], row: usize, pos: usize) -> bool {
    if pos == 0 {
        false
    } else {
        data[row][pos - 1].is_ascii_digit()
    }
}

fn check_right(data: &[Vec<char>], row: usize, pos: usize) -> bool {
    if pos == data[row].len() {
        false
    } else if pos == data[row].len() - 1 {
        data[row][pos].is_ascii_digit()
    } else {
        data[row][pos + 1].is_ascii_digit()
    }
}

fn concat_buffers(mut left: Vec<char>, digit: &Vec<char>, right: &Vec<char>) -> Vec<u32> {
    let mut parts: Vec<u32> = Vec::new();
    if digit.is_empty() {
        if !left.is_empty() {
            left.reverse();
            let part: u32 = left.iter().collect::<String>().parse::<u32>().unwrap();
            parts.push(part);
        }
        if !right.is_empty() {
            let part: u32 = right.iter().collect::<String>().parse::<u32>().unwrap();
            parts.push(part);
        }
    } else if !left.is_empty() && !digit.is_empty() && !right.is_empty() {
        left.reverse();
        let part: u32 = left
            .iter()
            .chain(digit.iter())
            .chain(right.iter())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        parts.push(part);
    } else if !left.is_empty() && !digit.is_empty() {
        left.reverse();
        let part: u32 = left
            .iter()
            .chain(digit.iter())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        parts.push(part);
    } else {
        let part: u32 = digit
            .iter()
            .chain(right.iter())
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        parts.push(part);
    }
    parts
}

#[cfg(test)]
mod tests {
    use crate::part_2;
    #[test]
    fn sample_input() {
        let answer = part_2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(answer, "467835".to_string())
    }
}
