#![allow(dead_code)]

#[macro_use]
extern crate log_macro;

mod run;

fn main() {
    run::mdka();
    log!("done");
}
