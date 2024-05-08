#!/usr/bin/env -S cargo +nightly -Zscript
// TODO: run this rust_script.rs somehow
// have many of these rust_scripts in scripts folder
// I can compile/run from some command
// ---
// [dependencies]
// rust_utils = { path = "../../crates/rust_utils" }
// ---
use log_macro::log;

fn main() {
    let a = rust_utils::add(2, 2);
    log!(a);
}
