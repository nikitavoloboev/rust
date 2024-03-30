#![allow(dead_code)]

#[macro_use]
extern crate log_macro;

mod run;

fn main() {
    let url = "https://github.com/teamhanko/hanko/blob/main/frontend/elements/README.md";
    match run::html_from_url(url) {
        Ok(markdown) => {
            println!("{}", markdown);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
