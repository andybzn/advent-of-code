use std::fs;
use std::io::{BufRead, Error};
use std::collections::HashMap;

fn main() -> Result<(),Error>{

    // figure out the priorities
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut priorities: HashMap<&char, isize> = HashMap::new();
    for i in alphabet.iter().enumerate().take(52) {
        let d: isize = i.0.try_into().unwrap();
        priorities.insert(i.1, d + 1);
    };

    // storage
    let mut priority: isize = 0;
    let mut b_priority: isize = 0;

    // get puzzle input
    let input: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let input: Vec<String> = input.unwrap();

    // part 1 juju
    for rucksack in &input {

        // split the strings
        let rucksack: Vec<char> = rucksack.chars().collect();
        let comp_1: &[char] = &rucksack[0..(rucksack.len()/2)];
        let comp_2: &[char] = &rucksack[(rucksack.len()/2)..];

        let mut contents: Vec<char> = Vec::new(); // vec to hold common chars

        // compare letters
        for l in comp_1.iter(){
            if comp_2.contains(l) && !(contents.contains(l)) {
                contents.push(*l);
            };
        };

        // calculate char values
        for c in contents.iter() {
            priority += priorities.get(c).unwrap();
        };
    };

    // part 2 juju
    for group in 0..(input.len()/3) {
        let group:usize = group * 3;
        let e1: Vec<char> = input[group].chars().collect();
        let e2: Vec<char> = input[group + 1].chars().collect();
        let e3: Vec<char> = input[group + 2].chars().collect();
        let mut badge: Vec<char> = Vec::new();

        for c in e1 {
            if e2.contains(&c) && e3.contains(&c) && !(badge.contains(&c)){
                badge.push(c);
            }
        }

        b_priority += priorities.get(&badge[0]).unwrap();
    }

    //output
    println!("Part One - Priority sum: {}", priority);
    println!("Part One - Badge priority sum: {}", b_priority);
    Ok(())
}
