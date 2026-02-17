fn factorial(num: usize) -> usize {
    if num == 1 || num == 0 {
        1
    } else {
        factorial(num - 1) * num
    }
}

fn main() {
    println!("{}", factorial(5))
}