use std::collections::{LinkedList};
use std::io::{stdin, stdout, Write};

fn push(ll: &mut LinkedList<usize>) {
    let mut a: String = String::new();

    print!("Enter the value: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut a)
        .expect("failed to read line");

    let a: usize = a.trim()
        .parse::<usize>()
        .unwrap();

    ll.push_back(a);
}

fn pop(ll: &mut LinkedList<usize>) {
    if ll.is_empty() {
        println!("Empty");
    } else {
        println!("Popped element: {:?}", ll.pop_back().unwrap());
    }
}

fn display(ll: &LinkedList<usize>) {
    if ll.is_empty() {
        println!("Empty");
    } else {
        println!("The elements are: {:?}", ll.iter().collect::<Vec<_>>());
    }
}

fn main() {
    let mut ll: LinkedList<usize> = LinkedList::new();

    loop {
        println!("=== Main Menu ===");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Display");
        println!("4. Exit");
        print!("Enter the choice: ");
        stdout().flush().unwrap();
        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("failed to read line");
        let choice: usize = choice.trim()
            .parse::<usize>()
            .unwrap();

        match choice {
            1 => {
                push(&mut ll);
            },
            2 => {
                pop(&mut ll);
            },
            3 => {
                display(&ll);
            },
            4 => {
                println!("Exiting...");
                break
            },
            _ => {
                println!("Invalid choice");
            }
        }
    }
}