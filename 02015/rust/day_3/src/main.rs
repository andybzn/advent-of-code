// imports
use std::io::Error;
use std::fs;

// main
fn main() -> Result<(), Error> {

    // parse the input
    let input = fs::read_to_string("input.txt")?;
    let input: Vec<char> = input.chars().collect(); // split to a char array

    // counters & vectors
    let mut move_count: i16 = 0;
    let mut regular_santa: Vec<[i16; 2]> = Vec::new();
    let mut robot_santa: Vec<[i16; 2]> = Vec::new();

    // prime vectors
    regular_santa.push([0,0]);
    robot_santa.push([0,0]);

    // logic
    for direction in input {

        if move_count % 2 == 0 {
          // regular_santa
          let reg_v: i16 = regular_santa.last().unwrap()[0];
          let reg_h: i16 = regular_santa.last().unwrap()[1];
          let new_coords: (i16, i16) = navigate(reg_v, reg_h, direction);
          regular_santa.push([new_coords.0, new_coords.1]);
        } else {
          // robo santa
          let robo_v: i16 = robot_santa.last().unwrap()[0];
          let robo_h: i16 = robot_santa.last().unwrap()[1];
          let new_coords: (i16, i16) = navigate(robo_v, robo_h, direction);
          robot_santa.push([new_coords.0, new_coords.1]);
        };

        // track moves
        move_count += 1;
    };

    // combine vectors
    regular_santa.append(&mut robot_santa);

    // dedup coords
    regular_santa.sort();
    regular_santa.dedup();

    // output
    println!("move count: {:?}", move_count);
    println!("total houses visited: {:?}", regular_santa.len());

    Ok(())
}

fn navigate(vertical: i16, horizontal: i16, instruction: char) -> (i16, i16) {
  // take current up_down left_right for each persona, and current move value
  // apply move and return coords

  let mut vertical: i16 = vertical;
  let mut horizontal: i16 = horizontal;

  if instruction == '^' {
    vertical += 1;
  } else if instruction == 'v'  {
    vertical -= 1;
  } else if instruction == '<'  {
    horizontal += 1;
  } else if instruction == '>' {
    horizontal -= 1;
  } else {
    // nothing
  };

  (vertical, horizontal)
}