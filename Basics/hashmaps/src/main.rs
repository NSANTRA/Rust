use std::collections::HashMap;

fn basics() {
    let mut map: HashMap<usize, String> = HashMap::new();

    map.insert(1, String::from("One"));

    map.insert(2, String::from("Two"));

    println!("Hashmap: {:?}", map);
    println!("Is 1 in the keys: {}", map.contains_key(&1));
    println!("Is 3 in the keys: {}", map.contains_key(&3));

    println!("The value of key 1 is: {}", map.get(&1).unwrap());

    map.entry(2).or_insert(String::from("Two Two"));
    map.entry(3).or_insert(String::from("Three"));

    println!("Updated HashMap: {:?}", map);

    for key in map.keys() {
        println!("Map_1 Key: {}", key);
    }

    for value in map.values() {
        println!("Map_1 Key: {}", value);
    }

    let key_vector: Vec<usize> = map.clone().into_keys().collect();

    println!("Key Vector: {:?}", key_vector);

    let value_vector: Vec<String> = map.clone().into_values().collect();

    println!("Value Vector: {:?}", value_vector);

    for (k, v) in map.iter() {
        println!("Map_1 Key: {}, Map_1 Value: {}", k, v);
    }

    for (k, v) in map.iter_mut() {
        println!("Map_1 Key: {}, Map_1 Value (Changed): {}", k, String::from("Changed value"));
    }

    // Alternate way of initializing HashMap
    let map_2: HashMap<String, String> = HashMap::from([
        (String::from("One"), String::from("Review 1")),
        (String::from("Two"), String::from("Review 2")),
        (String::from("Three"), String::from("Review 3")),
    ]);

    println!("{:?}", map_2);
}

#[derive(Hash, PartialEq, Debug, Eq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new_viking(name: &str, country: &str) -> Viking {
        Viking {
            name: String::from(name),
            country: String::from(country)
        }
    }
}

fn main() {
    basics();

    let value = [
        (Viking::new_viking("Thor", "Norway"), 1),
        (Viking::new_viking("Loki", "Norway"), 2),
        (Viking::new_viking("Kraven", "Valheim"), 3),
    ];

    let vikings = HashMap::from(value);

    println!("Viking HashMap: {:?}", vikings);

    println!("Kraven Rank: {:?}", vikings.get(&Viking::new_viking("Kraven", "Valheim")).unwrap());
}

