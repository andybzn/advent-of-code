// imports
use std::io::{Error, BufRead};
use std::fs;

fn main() -> Result<(), Error> {

    // parse input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // storage
    let queued_wires = Vec::new();
    let wire_values = Vec::new();

    // parse the wires
    for i in input {
        println!("{}",&i);

        // check to see if the wire has a value else add to buffer;
        let wire = &i.split(' ');
        if !(wire_values.contains(wire[0])) {
            queued_wires.push(i);
        } else {
            // process

        };
    };

    Ok(())
}
