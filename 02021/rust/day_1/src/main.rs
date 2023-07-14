use std::fs;
use std::io::BufRead;

fn main() {
    let input: Vec<i32> = fs::read("../../inputs/day_1.txt").expect("Couldn't read file")
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut counter: i32 = 0;
    let mut alt_counter: i32 = 0;
    let mut prev_sum: i32 = 0;

    for measurement in input.iter().enumerate() {
        if measurement.0 != 0 && measurement.1 > &input[measurement.0 - 1] {
            counter += 1;
        }

        let sum: i32 = input.get(measurement.0).unwrap_or(&0) +
            input.get(measurement.0 + 1).unwrap_or(&0) +
            input.get(measurement.0 + 2).unwrap_or(&0);
        if measurement.0 != 0 && sum > prev_sum {
            alt_counter += 1;
        }
        prev_sum = sum;
    }

    println!("Part 1: {counter}");
    println!("Part 2: {alt_counter}");
}