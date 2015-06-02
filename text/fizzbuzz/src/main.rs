fn main() {
    for index in 1..101 {
        match (index % 3, index % 5) {
            (0, 0) => println!("FIZZBUZZ!"),
            (0, _) => println!("FIZZ"),
            (_, 0) => println!("BUZZ"),
            _ => println!("{}", index)
        }

    }
}
