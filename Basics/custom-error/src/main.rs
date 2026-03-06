use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Write, stdin, stdout};

#[derive(Debug)]
enum CustomError {
    InvalidItem,
    ItemNotFound,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CustomError::InvalidItem => {
                writeln!(f, "Item Invalid")
            }
            CustomError::ItemNotFound => {
                writeln!(f, "Item Not Found")
            }
        }
    }
}

impl Error for CustomError {}

fn check(item_id: isize) -> Result<String, CustomError> {
    if item_id == 0 {
        Err(CustomError::InvalidItem)
    } else if item_id == 42 {
        Ok(String::from("Item Found"))
    } else {
        Err(CustomError::ItemNotFound)
    }
}

fn main() {
    let mut input: String = String::new();

    print!("Enter item ID: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let input: isize = input.trim().parse::<isize>().unwrap();

    match check(input) {
        Ok(value) => {
            println!("{}", value);
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
