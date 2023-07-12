use std::fs;

fn main() {
    // parse input
    let input = fs::read_to_string("input.txt").expect("Couldn't read file");
    let input: Vec<char> = input.chars().collect();

    let mut move_count: i16 = 0;
    let mut regular_santa: Vec<(i16, i16)> = vec![(0, 0)];
    let mut robot_santa: Vec<(i16, i16)> = vec![(0, 0)];

    for direction in input {
        if move_count % 2 == 0 {
            let coords: (i16, i16) = get_coords(&regular_santa);
            regular_santa.push(navigate(coords.0, coords.1, direction));
        } else {
            let coords: (i16, i16) = get_coords(&robot_santa);
            robot_santa.push(navigate(coords.0, coords.1, direction));
        };
        move_count += 1;
    }

    regular_santa.append(&mut robot_santa);
    regular_santa.sort_unstable();
    regular_santa.dedup();

    println!("Part 1: {move_count}");
    println!("Part 2: {:?}", regular_santa.len());
}

fn get_coords(vec: &Vec<(i16, i16)>) -> (i16, i16) {
    vec[vec.len() - 1]
}

fn navigate(mut vertical: i16, mut horizontal: i16, instruction: char) -> (i16, i16) {
    match instruction {
        '^' => vertical += 1,
        'v' => vertical -= 1,
        '<' => horizontal += 1,
        '>' => horizontal -= 1,
        _ => (),
    }

    (vertical, horizontal)
}
