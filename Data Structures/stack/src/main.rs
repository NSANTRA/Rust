use std::io::{stdin, stdout, Write};

fn push(stack: &mut Vec<usize>, size: &usize) {
    if stack.len() < *size {
        let mut input: String = String::new();
        print!("Element to push: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: usize = input
            .trim()
            .parse::<usize>()
            .unwrap();

        stack.push(input);
    } else {
        println!("Stack overflow!");
    }
}

fn pop(stack: &mut Vec<usize>) {
    if stack.len() <= 0  {
        println!("Stack underflow!");
    } else {
        let a = stack.pop().unwrap();
        println!("Popped element: {}", a);
    }
}

fn peek(stack: &mut Vec<usize>) {
    if stack.len() <= 0 {
        println!("Stack underflow!");
    } else {
        println!("Topmost element: {}", stack[stack.len() - 1]);
    }
}

fn display(stack: &Vec<usize>) {
    if stack.len() <= 0 {
        println!("Stack underflow!");
    } else {
        println!("The current stack is {:?}", stack);
    }
}

fn main() {
    let mut stack: Vec<usize> = Vec::new();

    let mut size: String = String::new();

    print!("Enter your stack size: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut size)
        .expect("Failed to read input");

    let size: usize = size
        .trim()
        .parse::<usize>()
        .unwrap();

    loop {
        println!("=== Main Menu ===");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Display");
        println!("5. Exit");

        print!("Enter your choice: ");
        stdout().flush().unwrap();
        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: usize = choice
            .trim()
            .parse::<usize>()
            .expect("Please type a number!");

        match choice {
            1 => {
                push(&mut stack, &size);
            },
            2 => {
                pop(&mut stack);
            },
            3 => {
                peek(&mut stack);
            },
            4 => {
                display(&stack);
            },
            5 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Wrong choice!");
            }
        }
    }
}