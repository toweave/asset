use clap::{Parser};
use crate::creates::create::Creates;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(name = "asset")]
#[command(author = "Toweave Lee. <toweave@163.com>")]
#[command(version = "1.0")]
#[command(about = "Simple resource management command line.")]
#[command(long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub name: Option<String>,

    // #[arg(short, long)]
    #[command(subcommand)]
    pub create: Option<Creates>,
}