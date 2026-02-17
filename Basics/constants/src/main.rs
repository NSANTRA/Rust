const NUM: usize = 16;
static mut STR: &str = "Hello, world!";

fn main() {
    println!("const: {NUM}");
    // NUM = 17;                            // Cannot modify a const variable, else Rust panics
    unsafe { STR = "Hello, Rust" };         // Need to use unsafe annotation in case of a mutable static. Rust considers mutable static as unsafe
    let s = unsafe {STR};
    println!("static: {s}");
}