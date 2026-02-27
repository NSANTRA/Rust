use std::io::{Write, stdin, stdout};

fn func(drink: Option<&str>) {
    match drink {
        Some("lemonade") => {
            println!("Wow, you like lemonade");
        }
        Some("Cola") => {
            println!("So you like Cola, you junkie");
        }
        Some(other) => {
            println!("You drink {other}");
        }
        None => {
            println!("No drinks. Cool");
        }
    }
}

fn main() {
    let mut drink: String = String::new();

    print!("Enter the name of your drink: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut drink)
        .expect("Failed to read the line");
    
    let drink: String = drink.trim().parse().unwrap();

    if drink.is_empty() {
        func(None);
    } else {
        func(Some(&drink));
    }
}
