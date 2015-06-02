use std::io;

fn reverse_word(word: &mut String) {
    let mut x: Vec<char> = word.trim().chars().collect();
    x.reverse();
    *word = x.iter().map(|&c| c).collect();
}


fn main() {
    println!("Enter a word");

    let mut word = String::new();

    io::stdin().read_line(&mut word)
        .ok()
        .expect("Cannot read input");

    reverse_word(&mut word);
    println!("Reversed word is: {}", word);

}
