use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // handle input
    let input = fs::read_to_string("input.txt")?;
    let input: Vec<char> = input.chars().collect();

    // storage
    let mut i: usize = 0;
    let mut matched: bool = false;
    let mut matched_pos: usize = 0;
    let mut x: usize = 0;
    let mut message_match: bool = false;
    let mut message_pos: usize = 0;

    // part one juju - i just had fun with match for this one
    while (i < input.len() - 4) && !matched {
        let c1 = input[i];
        let c2 = input[i + 1];
        let c3 = input[i + 2];
        let c4 = input[i + 3];

        matched = match (c1, c2, c3, c4) {
            (c1, c2, c3, c4) if c1 == c2 || c1 == c3 || c1 == c4 => false,
            (c1, c2, c3, c4) if c2 == c1 || c2 == c3 || c2 == c4 => false,
            (c1, c2, c3, c4) if c3 == c1 || c3 == c2 || c3 == c4 => false,
            (c1, c2, c3, c4) if c4 == c1 || c4 == c2 || c4 == c3 => false,
            _ => true,
        };

        if matched {
            matched_pos = i + 4;
        };
        i += 1;
    }

    // part two juju - match would be trash to write here so let's do a slice
    while (x < input.len() - 14) && !message_match {
        let mut slice: Vec<char> = input[x..=x + 13].to_vec();
        let len: usize = slice.len();

        slice.sort();
        slice.dedup();

        if slice.len() == len {
            message_match = true;
            message_pos = x + 14;
        };
        x += 1;
    };

    // output
    println!("Part One - start-of-packet marker @: {}", matched_pos);
    println!("Part Two - start-of-message marker @: {}", message_pos);
    Ok(())
}
