use std::io::{Write, stdin, stdout};
use std::num::ParseIntError;

fn multiply(first: &str, second: &str) -> Result<usize, ParseIntError> {
    match first.trim().parse::<usize>() {
        Ok(first) => match second.trim().parse::<usize>() {
            Ok(second) => Ok(first * second),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn display(result: Result<usize, ParseIntError>) {
    match result {
        Ok(product) => {
            println!("The product of the given 2 numbers are: {product}");
        }
        Err(err) => {
            println!("Error encountered: {err}");
        }
    }
}

type ParseResult<T> = Result<T, ParseIntError>;

fn addition(first: &str, second: &str) -> ParseResult<usize> {
    match first.trim().parse::<usize>() {
        Ok(first) => match second.trim().parse::<usize>() {
            Ok(second) => Ok(first + second),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn display_alias(result: ParseResult<usize>) {
    match result {
        Ok(number) => {
            println!("The addition of the given 2 numbers are: {number}");
        }
        Err(err) => {
            println!("Error encountered while addition: {err}");
        }
    }
}

fn main() {
    let mut first: String = String::new();
    print!("Enter the first number: ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut first);

    let mut second: String = String::new();
    print!("Enter the second number: ");
    stdout().flush().unwrap();
    let _ = stdin().read_line(&mut second);

    let result = multiply(&first, &second);
    display(result);

    let result = addition(&first, &second);
    display_alias(result);
}
