#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod app {
    mod config;
    mod crypto;
    mod dto;
    mod error;
    mod repo;
}

fn main() {
    println!("Hello, world!");
}
