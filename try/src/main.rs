#![allow(dead_code)]

use log_macro::log;

fn main() {
    let animals = vec!["dog"];
    log!(animals);
    log!("test");
}
