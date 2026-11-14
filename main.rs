use std::{io::{self, BufRead}};

// Count the vowels (a, e, i, o, u, in either case) in the borrowed text.
fn vowel_count(s: &str) -> usize {
    let mut n = 0;
    for c in s.chars(){
        if c == 'a' || c== 'A' || c== 'e' || c=='E' || c == 'i' || c == 'I' || c == 'o' || c == 'O' || c == 'u' || c == 'U'{
            n+=1;
        }
    }
    n
}

// Change the caller's String in place: make it uppercase, then make sure
// it ends with '!' (add one only if it does not already end in '!').
fn make_loud(s: &mut String) {
    // TODO: modify the String behind the mutable reference
    if s.ends_with('!'){
    }else {
        s.push('!');
    }
    *s=s.to_uppercase().to_string();
}

fn main() {
    let stdin = io::stdin();
    let mut line = stdin.lock().lines().next().unwrap().unwrap();
    let before = vowel_count(&line);
    make_loud(&mut line);
    println!("{}", before);
    println!("{}", line);
    println!("{}", vowel_count(&line));
}
