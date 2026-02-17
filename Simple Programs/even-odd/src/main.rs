use std::io::{stdin, stdout, Write};

fn is_even(num: &usize) -> bool {
    if *num % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let mut num: String = String::new();
    print!("Enter the number: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: usize = num
        .trim()
        .parse::<usize>()
        .unwrap();

    if is_even(&num) {
        println!("Even");
    } else {
        println!("Odd");
    }
}