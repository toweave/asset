// use std::error::Error;
use clap::{Subcommand, ValueEnum};
use crate::structs::root::Args;

#[derive(Debug, Subcommand)]
pub enum Creates {
    /// Create file or folder to current path.
    Create {
        mode: Mode,

        #[arg(short, long)]
        name: String
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Run swiftly
    Folder,
    /// Crawl slowly but steadily
    ///
    /// This paragraph is ignored because there is no long help text for possible values.
    File,
}

pub fn run (args: &Args){
    println!("Hello {}!", &args.name.clone().unwrap_or("world".into()));

    match &args.create {
        Some(Creates::Create { mode, name}) => {
            println!("'myapp add' was used, name is: {:?}", mode);
            match mode {
                Mode::Folder => {
                    println!("Folder {}", name);
                }
                Mode::File => {
                    println!("File {}", name);
                }
            }

        }
        None => {}
    }
}


// match &args.create {
//   Some(Creates::Create { mode, name}) => {
//       println!("'myapp add' was used, name is: {:?}", mode);
//       match mode {
//           Mode::Folder => {
//               println!("Folder {}", name);
//           }
//           Mode::File => {
//               println!("File {}", name);
//           }
//       }

//   }
//   None => {}
// }