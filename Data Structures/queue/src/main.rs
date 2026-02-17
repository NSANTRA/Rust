use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

fn enqueue(queue: &mut VecDeque<usize>, size: &usize) {
    if queue.len() >= *size {
        println!("Queue full")
    } else {
        let mut input: String = String::new();
        print!("Enter the element to enqueue: ");
        stdout().flush().unwrap();
        stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input: usize = input
            .trim()
            .parse::<usize>()
            .unwrap();

        queue.push_back(input);
    }
}

fn dequeue(queue: &mut VecDeque<usize>) {
    if queue.len() == 0 {
        println!("Queue is empty");
    } else {
        let a = queue.pop_front().unwrap();
        println!("Dequeued element is {}", a);
    }
}

fn peek(queue: &mut VecDeque<usize>) {
    if queue.len() == 0 {
        println!("Queue is empty");
    } else {
        println!("Peeked element is {}", queue[0]);
    }
}

fn display(queue: &VecDeque<usize>) {
    if queue.len() == 0 {
        println!("Queue is empty");
    } else {
        println!("The current queue is {:?}", queue);
    }
}

fn main() {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut size: String = String::new();

    print!("Enter the size of the queue: ");
    stdout().flush().unwrap();
    stdin()
        .read_line(&mut size)
        .expect("Error reading input");

    let size: usize = size
        .trim()
        .parse::<usize>()
        .unwrap();

    loop {
        println!("=== Main Menu ===");
        println!("1. Enqueue");
        println!("2. Dequeue");
        println!("3. Peek");
        println!("4. Display");
        println!("5. Exit");
        print!("Enter your choice: ");
        stdout().flush().unwrap();
        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Error reading input");

        let choice: usize = choice
            .trim()
            .parse::<usize>()
            .unwrap();

        match choice {
            1 => enqueue(&mut queue, &size),
            2 => dequeue(&mut queue),
            3 => peek(&mut queue),
            4 => display(&queue),
            5 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Not a valid choice");
            }
        }
    }
}