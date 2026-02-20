#[derive(Debug)]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    active: bool,
}

#[derive(Debug)]
struct Tuple1(usize, f64, String);

#[derive(Debug)]
struct Tuple2(usize, String);

fn build_user(username: String, first_name: String, last_name: String) -> User {
    // If the function parameters and the struct field names are same, we can use the shorthand
    // syntax.
    User {
        username,
        first_name,
        last_name,
        active: true,
    }
}

fn main() {
    let mut user_1 = User {
        first_name: String::from("Neelotpal"),
        last_name: String::from("Santra"),
        active: true,
        username: String::from("NSANTRA"),
    };

    println!("Username: {}", user_1.username);
    println!("First Name: {}", user_1.first_name);
    println!("Last Name: {}", user_1.last_name);
    println!("Active Status: {}", user_1.active);

    println!("User_1: {:?}", user_1);

    user_1.active = false;

    println!("Updated User_1: {:?}", user_1);

    let user_2 = build_user(
        String::from("neelotpalsantra_"),
        String::from("Neelotpal"),
        String::from("Santra"),
    );

    println!("User_2: {:?}", user_2);

    let user_3 = User {
        first_name: String::from("Debasis"),
        username: String::from("DSANTRA"),
        ..user_1
    };

    println!("Obtained from updated user_1: {:?}", user_3);

    let tuple_1 = Tuple1(10, 20.3, String::from("Tuple1"));
    let tuple_2 = Tuple2(20, String::from("Tuple2"));

    println!("Tuple 1: {:?}", tuple_1);
    println!("Tuple 2: {:?}", tuple_2)
}
