// imports
use std::io::{Error, BufRead};
use std::fs;

//main
fn main() -> Result<(), Error> {

    // get input from file
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input = input.unwrap();

    // store totals
    let mut wrapping: u32 = 0;
    let mut ribbon: u32 = 0;

    // parse input
    for item in input {

        // split line and collect
        let split = item.split('x');
        let vec: Vec<&str> = split.collect();

        // parse
        let length: u32 = vec[0].to_string().parse::<u32>().unwrap();
        let width: u32 = vec[1].to_string().parse::<u32>().unwrap();
        let height: u32 = vec[2].to_string().parse::<u32>().unwrap();
        let mut sides: Vec<u32> = vec![length, width, height]; // for the ribbon

        // calculate wrapping
        let area_1: u32 = 2 * (length * width);
        let area_2: u32 = 2 * (width * height);
        let area_3: u32 = 2 * (height * length);
        let mut extra: Vec<u32> = vec![(length * width), (width * height), (height * length)]; // for the extra

        // calculate extra wrapping
        extra.sort();
        let total_extra: u32 = extra[0];

        // total wrapping
        let total_area: u32 = area_1 + area_2 + area_3 + total_extra;
        wrapping += total_area;

        // calculate ribbon & bows
        sides.sort();
        let wrap: u32 = sides[0] + sides[0] + sides[1] + sides[1];
        let bow: u32 = length * width * height; 

        //total ribbon
        let total_ribbon: u32 = wrap + bow;
        ribbon += total_ribbon;
        
    };

    // output
    println!("total wrapping required: {:?} sq. ft.", wrapping);
    println!("total ribbon required: {:?} ft.", ribbon);

    Ok(())
}