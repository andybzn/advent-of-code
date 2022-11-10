// imports
use std::fs;
use std::io::{BufRead, Error};
use fancy_regex::Regex as fancy;

fn main() -> Result<(), Error> {
    // parse input
    let words: Result<Vec<String>, Error> = (fs::read("input.txt")?).lines().collect();
    let words: Vec<String> = words.unwrap();

    // old word rules
    let vowels = fancy::new("[aeiouAEIOU]").unwrap();
    let naughty_strings = fancy::new("(ab|cd|pq|xy)").unwrap();
    let repeating_chars = fancy::new(r"(.)\1").unwrap();

    // new word rules
    let pairs = fancy::new(r"\w*?(\w{2})\w*?\1\w*?").unwrap();
    let repeats = fancy::new(r"(.)\w\1").unwrap();

    // counters
    let mut old_nice_words: i16 = 0;
    let mut new_nice_words: i16 = 0;

    // logic
    for word in words {

        // old words
        // filter out naughty words - could this be a match?
        if (naughty_strings.find_iter(&word).count() == 0)
            && (vowels.find_iter(&word).count() >= 3)
            && (repeating_chars.find_iter(&word).count() != 0)
        {
            old_nice_words += 1;
        };

        // new words
        if (pairs.find_iter(&word).count() > 0)
            && (repeats.find_iter(&word).count() > 0) 
        {
            new_nice_words += 1;
        };
    };

    // output
    println!("old nice strings: {:?}", old_nice_words);
    println!("new nice strings: {:?}", new_nice_words);

    Ok(())
}