use std::fs;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error> {
    // gather input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // stuff
    let mut count: usize = 0;
    let mut elves: Vec<i128> = Vec::new();
    elves.push(0);

    // calculate
    for cal in input {
        if cal.is_empty() {
            count += 1; // increase the tracker
            elves.push(0); // push a new blank to the vector
        } else {
            let cval: i128 = elves[count]; // grab the current count
            elves[count] = cval + cal.parse::<i128>().unwrap(); // do the math
        }
    }

    // sort & math
    elves.sort();
    let part_one = elves[elves.len() - 1];
    let part_two = elves[elves.len() - 1] + elves[elves.len() - 2] + elves[elves.len() - 3];

    // output
    println!("Part One - Largest calory value: {:?}", part_one);
    println!("Part Two - Top 3 calory value: {}", part_two);

    Ok(())
}
