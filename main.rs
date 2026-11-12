use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line: String = stdin.lock().lines().next().unwrap().unwrap();

    // TODO: take a trimmed &str view of `line`, then print three lines:
    //   1. that text in uppercase
    //   2. how many characters it has
    //   3. how many bytes it has
    let trimmed: &str=line.trim();
    let upper: String=trimmed.to_uppercase();
    let count=line.chars().count();
    let byte=line.len();
    println!("{}", upper);
    println!("{}", count);
    println!("{}", byte);
}
