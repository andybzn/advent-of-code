use std::fs;

fn main() {
    // parse the input
    let input = fs::read_to_string("../../inputs/day_1.txt").expect("Couldn't read input"); // read the file
    let input_split: Vec<char> = input.chars().collect(); // split to a char array

    // figure out the floor number
    let mut floor_num: i32 = 0; // prepare a counter
    let mut basement: bool = false; // basement check
    let mut basement_position: u32 = 1; // basement position tracker

    for floor in input_split {
        // floor logic
        match floor {
            '(' => floor_num += 1,
            ')' => floor_num -= 1,
            _ => (),
        };
        // basement logic
        if !basement {
            basement_position += 1;
            basement = matches!(floor_num, 0);
        };
    };

    println!("Part 1: {floor_num}");
    println!("Part 2: {basement_position}");
}