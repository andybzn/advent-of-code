// imports 
use std::io::Error;
use std::fs;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::str;

// main
fn main () -> Result<(), Error>{

    // parse input
    let key = fs::read_to_string("input.txt")?;

    // storage
    let mut num: i32 = 0;
    let mut current_hash;

    let mut five_complete = false;
    let mut winning_five_num = 0;
    let mut winning_five_hash: String = String::from("");

    let mut six_complete = false;
    let mut winning_six_num = 0;
    let mut winning_six_hash: String = String::from("");

    // logic
    while !((five_complete) && (six_complete)) {
        num += 1; // increment

        // find the hash
        let result = compute_hash(key.to_string(), num.to_string());
        current_hash = result;

        // check the hash
        if !five_complete && current_hash.clone()[..5] == *"00000" {
            winning_five_num = num;
            winning_five_hash = current_hash.clone();
            five_complete = true;
        };

        // check the hash
        if !six_complete && current_hash.clone()[..6] == *"000000" {
            winning_six_num = num;
            winning_six_hash = current_hash.clone();
            six_complete = true;
        };
    };

    // output
    println!("Computation Complete");
    println!("Five Zeros");
    println!("----------");
    println!("Hash output: {}", winning_five_hash);
    println!("Number input: {}", winning_five_num);
    println!("----------");
    println!("Six Zeros");
    println!("----------");
    println!("Hash output: {}", winning_six_hash);
    println!("Number input: {}", winning_six_num);
    println!("----------");

    Ok(())
}

fn compute_hash( secret: String, number: String ) -> String {

        let secret: String = secret;
        let number: String = number;
        let secret_key: String = secret + &number;

        let mut hashfn: Md5 = Md5::new();
        hashfn.input_str(&secret_key);
        let hash: &str = &hashfn.result_str();

        hash.to_string()
}
