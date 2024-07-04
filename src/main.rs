#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod algo;
mod blocks;
mod enums;
mod guessing_game;
mod structs;

fn main() {
    let opt = Some(String::from("Hello world"));

    match &opt {
        // _ became s
        Some(ref s) => println!("Some: {s}"),
        None => println!("None!"),
    };

    println!("{opt:?}");
}
