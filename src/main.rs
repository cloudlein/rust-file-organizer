use crate::cli::parse_args;

mod scanner;
mod grouping;
mod cli;
mod mover;
mod ui;
mod config;

fn main() {
   // println!("Rust File Organizer — development in progress...");

    parse_args();
}
