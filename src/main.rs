#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

mod algo;
mod blocks;
mod enums;
mod structs;

fn main() {
    algo::use_algos();
    blocks::use_block();
    structs::use_struct();
    enums::use_enum();

    let opt = Some(String::from("Hello world"));

    match &opt {
        // _ became s
        Some(ref s) => println!("Some: {s}"),
        None => println!("None!"),
    };

    println!("{opt:?}");
}
