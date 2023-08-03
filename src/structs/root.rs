
use clap::{Parser, Subcommand};
use crate::creates::create::{  Mode, create_mode };
// use crate::deletes::delete::Deletes;
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
    pub commands: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create file or folder to current path.
    Create {
        mode: Option<Mode>,

        #[arg(short, long)]
        name: String
    },

    Delete {
        mode: Option<Mode>,

        #[arg(short, long)]
        name: String
    },
}

pub fn run (args: &Args){
    println!("Hello {}!", &args.name.clone().unwrap_or("world".into()));

    if let Some(commands) = &args.commands {
        match commands {
            Commands::Create { mode, name} => {
                create_mode(mode, name);
            }
            Commands::Delete { .. } => {
                println!("Commands::Delete");
            }
        }
    }

}