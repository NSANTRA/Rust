fn loop_statement() {
    let mut iterator: usize = 0;
    let result = loop {
        if iterator == 5 {
            break iterator;
        }

        iterator += 1;
    };

    println!("Result: {}", result)
}

fn while_statement() {
    let mut iterator: usize = 0;

    while iterator < 5 {
        println!("{}", iterator);
        iterator += 1;
    }
}

fn for_statement() {
    let a: [usize; 5] = [1, 2, 3, 4, 5];

    for i in a {
        println!("{}", i);
    }

    println!("-----------------");
    for i in (0..5).rev() {
        println!("{}", i);
    }
}

fn main() {
    loop_statement();
    println!("=================");
    while_statement();
    println!("=================");
    for_statement();
}