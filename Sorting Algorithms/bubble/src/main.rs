use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
};

fn sort(arr: &mut Vec<isize>) {
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn input_validation<T: FromStr>(num: String) -> Result<T, T::Err> {
    match num.trim().parse::<T>() {
        Ok(num) => Ok(num),
        Err(err) => Err(err),
    }
}

fn size_input() -> String {
    let mut num: String = String::new();
    print!("Enter the size of the array: ");
    stdout().flush().unwrap();

    let _ = stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");

    num
}

fn value_input(arr: &mut Vec<isize>, size: usize) {
    for _ in 0..size {
        let mut value: String = String::new();
        print!("Enter the size of the array: ");
        stdout().flush().unwrap();

        let _ = stdin()
            .read_line(&mut value)
            .expect("Failed to read the line");

        match input_validation::<isize>(value) {
            Ok(value) => {
                arr.push(value);
            }
            Err(err) => {
                println!("Please enter a valid integer: {err}");
            }
        }
    }
}

fn main() {
    let size: String = size_input();
    match input_validation::<usize>(size) {
        Ok(size) => {
            if size > 0 {
                let mut arr: Vec<isize> = Vec::with_capacity(size);

                value_input(&mut arr, size);
                println!("Before: {:?}", arr);
                sort(&mut arr);
                println!("After: {:?}", arr);
            }

            else {
                println!("Array length cannot be 0.");
            }
        }
        Err(err) => {
            println!("Please enter a valid integer: {err}");
        }
    }
}
