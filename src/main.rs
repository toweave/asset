use clap::{Parser};
use asset::structs::root::{Args, run};

fn main() {
    let args = Args::parse();
    run(&args);
}