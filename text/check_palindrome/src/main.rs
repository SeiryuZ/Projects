extern crate regex;

use std::io;
use std::ascii::AsciiExt;

use regex::Regex;

fn reverse_word(word: &String) -> String {
    let mut x: Vec<char> = word.trim().chars().collect();
    x.reverse();
    x.iter().map(|&c| c).collect()
}

fn main() {
    println!("Enter a word");

    let mut word = String::new();

    io::stdin().read_line(&mut word)
        .ok()
        .expect("Cannot read input");

    // Clean the word
    word = word.trim().to_string().to_ascii_lowercase();
    let re = Regex::new(r"[^\w]+").ok().expect("Regex fail");
    word = re.replace_all(&word, "");

    println!("Checking word '{}' for a palindrome", word);

    if reverse_word(&word) == word {
        println!("{} is a palindrome", word);
    }
    else{
        println!("{} is NOT a palindrome", word);
    }
}
