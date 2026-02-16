fn basic() {
    let x: i64 = 5;             // Immutable variable
    println!("Immutable: {x}");

    let mut y: i64 = 5;
    println!("Mutable (Step 1): {y}");            // Mutable variable
    y = 6;
    println!("Mutable (Step 2): {y}");
}

fn shadowing() {
    let x: i64 = 5;
    println!("Initialization: {x}");

    {
        let x: i64 = x * 2;
        println!("Inside Scope: {x}");
    }

    let x: i64 = x + 1;
    println!("Outside Scope: {x}");
}

fn main() {
    println!("Basics");
    basic();

    println!("-------------------------");

    println!("Shadowing");
    shadowing();
}