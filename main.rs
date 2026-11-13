use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32 = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    // TODO: choose between FizzBuzz, Fizz, Buzz and the number itself.
    if n%3==0 && n%5==0{
        println!("FizzBuzz");
    }
    else if n%3==0{
        println!("Fizz");
    }
    else if n%5==0 {
        println!("Buzz");
    }
    else {
        print!("{}", n);
    }
}
