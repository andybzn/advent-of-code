use std::fs;
use std::io::{BufRead, Error};

fn main() -> Result<(), Error>{

    // handle input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // storage
    let mut count: i128 = 0;
    let mut count_2: i128 = 0;

    // juju
    for line in input {
        let sections: Vec<&str> = line.split(',').collect(); // get the sections

        let mut ranges: Vec<Vec<i8>> = Vec::new(); // ranges for checking

        // build a range from each section
        for section in sections {
            let section: Vec<&str> = section.split('-').collect();
            let range_s: i8 = section[0].parse().unwrap();
            let range_e: i8 = section[1].parse().unwrap();

            let mut range: Vec<_> = Vec::new();
            for i in range_s..=range_e {
                range.push(i);
            }

            ranges.push(range);
        }

        // Part 1 juju - check if ranges contain each other entirely
        if ranges[0].iter().all(|x| ranges[1].contains(x)) ||
            ranges[1].iter().all(|x| ranges[0].contains(x)) {
                count += 1;
        }

        // Part 2 juju - check if the ranges overlap at all
        let mut overlap: bool = false;
        for a in ranges[0].iter() {
            if !(overlap) && ranges[1].contains(a){
                count_2 += 1;
                overlap = true;
            }
        }
    }

    // output
    println!("Part One - Duplicated work count: {}", count);
    println!("Part Two - Overlap  count: {}", count_2);

    Ok(())
}
