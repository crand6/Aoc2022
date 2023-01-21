// Make clippy as annoying as possible so I learn stuff
#![feature(iter_array_chunks)]
#![feature(get_many_mut)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
// But not that annoying because holy crap
#![allow(
    clippy::expect_used,
    clippy::arithmetic_side_effects,
    clippy::integer_arithmetic,
    clippy::implicit_return,
    clippy::wildcard_enum_match_arm,
    clippy::print_stdout,
    clippy::shadow_reuse,
    clippy::unwrap_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::blanket_clippy_restriction_lints,
    clippy::cargo_common_metadata
)]


/// Get library code specific to the day's challenges
mod day7;
use day7 as day;
use clap::{Command, Arg};

fn main() {
    let args = Command::new("Advent of Code")
    .arg(
        Arg::new("file").required(true)
    )
    .get_matches();

    let filename = args.get_one::<String>("file").unwrap();
    let data = std::fs::read_to_string(filename);

    if let Err(msg) = data {
        println!("Unable to open {filename}: {msg}");
        std::process::exit(1);
    }

    let data = data.unwrap();

    println!("Part A:");
    day::a(&data);

    println!("Part B:");
    day::b(&data);

    day::b("bvwbjplbgvbhsrlpgdmjqwftvncz");
}