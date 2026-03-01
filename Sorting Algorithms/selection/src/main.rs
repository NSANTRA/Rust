use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
};

fn sort(arr: &mut Vec<isize>) {
    for i in 0..arr.len() - 1 {
        let mut min: usize = i;

        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }

        arr.swap(i, min);
    }
}

fn display(arr: &Vec<isize>, mode: &str) {
    println!("{mode}: {:?}", arr);
}

fn input_validation<T: FromStr>(num: String) -> Result<T, T::Err> {
    match num.trim().parse::<T>() {
        Ok(num) => Ok(num),
        Err(err) => Err(err),
    }
}

fn main() {
    let mut num: String = String::new();
    print!("Enter the maximum number of elements: ");
    stdout().flush().unwrap();

    let _ = stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");

    match input_validation::<usize>(num) {
        Ok(num) => {
            if num > 0 {
                let mut arr: Vec<isize> = Vec::new();

                for _ in 0..num {
                    let mut input: String = String::new();
                    print!("Enter the element value: ");
                    stdout().flush().unwrap();

                    let _ = stdin()
                        .read_line(&mut input)
                        .expect("Failed to read the line");

                    match input_validation::<isize>(input) {
                        Ok(input) => {
                            arr.push(input);
                        }
                        Err(err) => {
                            println!("Please enter a valid integer: {err}");
                        }
                    }
                }

                display(&arr, &"Before");
                sort(&mut arr);
                display(&arr, &"After");
            } else {
                println!("Array length should be atleast 1.");
            }
        }
        Err(err) => {
            println!("Please enter a positive integer: {err}");
        }
    }
}
