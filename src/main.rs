use clap::{Parser};
use asset::structs::root::Args;
use asset::creates::create;

fn main() {
    let args = Args::parse();
    create::run(&args);
}