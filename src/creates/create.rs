// use std::error::Error;
// use std::ffi::OsStr;
use std::fs::{ self, File };
use std::path::Path;
use clap::{Subcommand, ValueEnum};
use crate::structs::root::Args;
use nu_ansi_term::Color::{
    Red,
    Blue,
    // Cyan,
    // Yellow
};

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
                    println!("name = {}", name);
                    match o {
                        Mode::Folder => {
                            // 创建文件夹
                            let path = Path::new(name);
                            if path.exists() {
                                println!("Folder {} 已存在", name);
                            } else {
                                let d = fs::create_dir_all(path);
                                match d {
                                    Ok(_f) => {
                                        println!("{} 文件夹创建成功", Blue.paint(name));
                                    }
                                    Err(err) => {
                                        println!("Error :: {:?}.", Red.paint(err.to_string()));
                                    }
                                }
                            }
                        }
                        Mode::File => {
                            // 创建文件
                            create_file_map(&name);
                        }
                    }
                }
                None => {
                    // 默认创建文件
                    create_file_map(&name);
                }
            }

        }
        None => {}
    }
}


pub fn create_file_map (name: &str) {
    let path = Path::new(name);
    let is_exists = path.exists();
    let is_file = path.is_file();
    let is_dir = path.is_dir();
    let is_relative = path.is_relative();
    let parent = path.parent();
    let file_name = path.file_name();
    let file_stem = path.file_stem();
    let extension = path.extension();
    // println!("{} is {}", Red.paint(name), is_file);
    println!("is_file = {}", is_file);
    println!("is_exists = {}", is_exists);
    println!("is_dir = {}", is_dir);
    println!("is_relative = {}", is_relative);
    println!("{:?} __ {:?} __ {:?} __ {:?}", parent, file_name, file_stem, extension);
    if is_exists {
        if is_dir {
            println!("{}", Red.paint("已存在同名文件夹"));
        }
    } else {
        match parent {
            Some(p) => {
                println!("parent.is_file = {}", p.is_file());
                println!("parent.is_exists = {}", p.exists());
                println!("parent.is_dir = {}", p.is_dir());
                if p.to_str() == Some("") || p.to_str() == Some(".") {
                    // 父级前路径为空，且文件不存在
                    create_file(name);
                } else {
                    if p.is_dir() {
                        create_file(name);
                    } else {
                        let d = fs::create_dir_all(p);
                        match d {
                            Ok(_f) => {
                                create_file(name);
                            }
                            Err(err) => {
                                println!("Error :: {:?}.", Red.paint(err.to_string()));
                            }
                        }
                    }
                }
            }
            None => {
                println!("parent = None");
            }
        }
    }
}

pub fn create_file (name: &str) {
    let file = File::create(name);
    match file {
        Ok(_f) => {
            // println!("file :: {:?}.", _f);
            println!("{} created successfully", Blue.paint(name));
        }
        Err(err) => {
            println!("Error :: {:?}.", err.to_string());
            println!("{} created failed", Red.paint(name));
        }
    }
}