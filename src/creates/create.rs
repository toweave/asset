// use std::error::Error;
use std::fs::File;
use std::path::Path;
use clap::{Subcommand, ValueEnum};
use crate::structs::root::Args;
use nu_ansi_term::Color::{ Red, Blue, Cyan, Yellow };

#[derive(Debug, Subcommand)]
pub enum Creates {
    /// Create file or folder to current path.
    Create {
        mode: Option<Mode>,

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
            match mode {
                Some(o) => {
                    println!("Folder {}", name);
                    match o {
                        Mode::Folder => {
                            // 创建文件夹
                            println!("Folder {}", name);
                        }
                        Mode::File => {
                            // 创建文件
                            create_file(&name);
                        }
                    }
                }
                None => {
                    // 默认创建文件
                    create_file(&name);
                }
            }

        }
        None => {}
    }
}


pub fn create_file (name: &str) {
    let is_exists = Path::new(name).exists();
    if is_exists {
        println!("{} is exists", Red.paint(name));
    } else {
        let file = File::create(name);
        match file {
            Ok(_f) => {
                // println!("file :: {:?}.", f);
                println!("{} created successfully", Blue.paint(name));
            }
            Err(err) => {
                println!("Error :: {:?}.", err.to_string());
                println!("{} created failed", Red.paint(name));
            }
        }
    }
}