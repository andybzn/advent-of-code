fn main() {
    println!("--- Day 10: Pipe Maze ---");
    let input: &str = include_str!("../input.txt");
    let parsed: (Vec<Vec<char>>, (usize, usize)) = parse_input(input);
    let answer: String = part_1(parsed.0, parsed.1);
    println!("Part 1: {answer}");
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    for (n, line) in grid.iter().enumerate() {
        if line.contains(&'S') {
            start = (n, line.iter().position(|x| x == &'S').unwrap())
        }
    }
    (grid, start)
}

fn part_1(input: Vec<Vec<char>>, starting_point: (usize, usize)) -> String {
    let total: u32 = walk_path(&input, starting_point, starting_point);
    format!("{}", total / 2)
}

fn walk_path(input: &Vec<Vec<char>>, position: (usize, usize), origin: (usize, usize)) -> u32 {
    let mut count: u32 = 0;
    if position != origin && input[position.0 as usize][position.1 as usize] == 'S' {
        count += 1;
    } else {
        let coords: (usize, usize) = get_next_coords(&input, position, origin);
        count += walk_path(&input, coords, position);
        count += 1;
    }
    count
}

fn get_next_coords(
    input: &Vec<Vec<char>>,
    position: (usize, usize),
    origin: (usize, usize),
) -> (usize, usize) {
    match input[position.0][position.1] {
        '|' => {
            let up: (usize, usize) = (position.0 - 1, position.1);
            let down: (usize, usize) = (position.0 + 1, position.1);
            if up != origin { up } else { down }
        }
        '-' => {
            let left: (usize, usize) = (position.0, position.1 - 1);
            let right: (usize, usize) = (position.0, position.1 + 1);
            if left != origin { left } else { right }
        }
        'L' => {
            let up: (usize, usize) = (position.0 - 1, position.1);
            let right: (usize, usize) = (position.0, position.1 + 1);
            if up != origin { up } else { right }
        }
        'J' => {
            let up: (usize, usize) = (position.0 - 1, position.1);
            let left: (usize, usize) = (position.0, position.1 - 1);
            if up != origin { up } else { left }
        }
        '7' => {
            let left: (usize, usize) = (position.0, position.1 - 1);
            let down: (usize, usize) = (position.0 + 1, position.1);
            if left != origin { left } else { down }
        }
        'F' => {
            let right: (usize, usize) = (position.0, position.1 + 1);
            let down: (usize, usize) = (position.0 + 1, position.1);
            if right != origin { right } else { down }
        }
        'S' => {
            let process_up: bool = position.0 != 0;
            let process_down: bool = position.0 != input.len();
            let process_left: bool = position.1 != 0;
            let process_right: bool = position.1 != input[0].len();
            let up: (usize, usize) = if process_up { (position.0 - 1, position.1) } else { (0, 0) };
            let down: (usize, usize) = if process_down { (position.0 + 1, position.1) } else { (0, 0) };
            let left: (usize, usize) = if process_left { (position.0, position.1 - 1) } else { (0, 0) };
            let right: (usize, usize) = if process_right { (position.0, position.1 + 1) } else { (0, 0) };

            if process_up && input[up.0][up.1] == 'F'
                || input[up.0][up.1] == '7'
                || input[up.0][up.1] == '|' {
                    up
            } else if process_down && input[down.0][down.1] == 'L'
                || input[down.0][down.1] == 'J'
                || input[down.0][down.1] == '|'
            {
                down
            } else if process_left && input[left.0][left.1] == 'F'
                || input[left.0][left.1] == 'L'
                || input[left.0][left.1] == '-'
            {
                left
            } else if process_right && input[right.0][right.1] == 'J'
                || input[right.0][right.1] == '7'
                || input[right.0][right.1] == '-'
            {
                right
            } else {
                (0, 0)
            }
        }
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_1};
    #[test]
    fn sample_input() {
        let input = parse_input(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(part_1(input.0, input.1), "4".to_string())
    }

    #[test]
    fn sample_input_obscured() {
        let input = parse_input(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        );
        assert_eq!(part_1(input.0, input.1), "4".to_string())
    }

    #[test]
    fn sample_input_two() {
        let input = parse_input(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(part_1(input.0, input.1), "8".to_string())
    }

    #[test]
    fn sample_input_two_obscured() {
        let input = parse_input(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ",
        );
        assert_eq!(part_1(input.0, input.1), "8".to_string())
    }

    #[test]
    fn actual_input() {
        let input: &str = include_str!("../input.txt");
        let parsed: (Vec<Vec<char>>, (usize, usize)) = parse_input(input);
        let answer: String = part_1(parsed.0, parsed.1);
        assert_eq!(answer, "6701".to_string())
    }
}
