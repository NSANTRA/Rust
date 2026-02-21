use std::collections::{HashSet};

fn main() {
    let mut set: HashSet<String> = HashSet::new();

    set.insert(String::from("Hello"));
    set.insert(String::from("World"));
    set.insert(String::from("I am Neelotpal"));

    println!("Hash Set: {:?}", set);

    for s in set.clone() {
        println!("{:?}", s);
    }

    println!("Hash Set size: {}", set.clone().len());

    let mut set_2: HashSet<Vec<usize>> = HashSet::new();

    set_2.insert(vec![1, 2]);
    set_2.insert(vec![2, 3]);

    println!("Hash set 2: {:?}", set_2);

    let mut set_3: HashSet<usize> = (0..8).collect();

    let even: HashSet<usize> = set_3.extract_if(|i| i % 2 == 0).collect();
    let odd: HashSet<usize> = set_3.extract_if(|i| i % 2 != 0).collect();

    println!("Hash set 3: {:?}", set_3);
    println!("Even HashSet: {:?}", even);
    println!("Odd HashSet: {:?}", odd);
}
