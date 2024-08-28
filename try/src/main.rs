// #![allow(dead_code)]
use log_macro::log;

// struct User {
//     username: String,
//     email: String,
//     signed_in_count: u64,
//     active: bool,
// }

fn main() {
    log!("hello");

    let animals = vec!["cat", "dog"];
    log!(animals);
    // dbg!(animals);

    let animals = vec!["cat", "dog"];
    let fish = vec!["salmon", "tuna"];
    log!(animals, fish);

    // structs();
    // log!("done");
}

// fn structs() {
//     let mut user1 = User {
//         email: String::from("nikita@nikiv.dev"),
//         username: String::from("nikiv"),
//         active: true,
//         signed_in_count: 1,
//     };
//     let name: String = user1.username;
//     user1.username = String::from("nikivi");
// }
