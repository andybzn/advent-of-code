// imports
use std::io::Error;
use std::fs;

// main
fn main() -> Result<(), Error>{

    // parse the input
    let input = fs::read_to_string("input.txt")?; // read the file
    let input_split: Vec<char> = input.chars().collect(); // split to a char array

    // figure out the floor number
    let mut floor_num: i128 = 0; // prepare a counter
    let mut basement: bool = false; // basement check
    let mut basement_position: u128 = 1; // basement position tracker

    for floor in input_split {

        // floor logic
        if floor == '('{
            floor_num += 1; // add a floor
        } else if floor == ')' {
            floor_num -= 1; // take a floor off
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "you're gonna be stuck in this elevator")); // error
        };

        // basement logic
        if !basement {
            basement_position += 1;
            basement = floor_check(floor_num)?;
        };
        
    };

    // output
    println!("Santa, you need floor: {}", floor_num);
    println!("By the way, you'll enter the basement at position: {}", basement_position);

    Ok(())
}

// floor check function
fn floor_check( floor:i128 ) -> Result<bool, Error>{

    let mut result: bool = false;

        if floor == 0 {
            result = true;
        } else if floor != 0 {
            result = false;
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "somehow both?")); // error
        };

    Ok( result )

}