use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use clap::Parser;

use klone::error::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The directory where the files to backup are
    origin_dir: PathBuf,
    /// The directory where you want to store the backup
    target_dir: PathBuf,
    /// Mark this if you want to create a new directory
    #[arg(short, long)]
    new: Option<String>,
}

// 1. Check if the paths are valid.
// 2. Check if you want to create a new path. If so check that the path does not exist and create
//    it.
// 3. Give to the library files only two valid path which it can work with directly.
// 4. Return the correspondent errors if neccesary.
fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);
    match args.origin_dir.try_exists() {
        Ok(exists) => {
            if !exists {
                let path = args.origin_dir.display().to_string();
                return Err(Error {
                    kind: ErrorKind::DirectoryDoesNotExist(path),
                });
            }
        }
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::ErrorReadingFS,
            })
        }
    }
    match args.target_dir.try_exists() {
        Ok(exists) => {
            if !exists {
                print!("The target directory does not exist. Do you want to create it? [Y/n]:  ");
                let _ = io::stdout().flush();
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("An error has happened");
                input.pop();
                println!("{:?}", input);
                match input.as_str() {
                    "y" | "Y" | "" => {
                        create_dir(args.target_dir.to_str().unwrap())?;
                    }
                    "n" | "N" => {
                        return Err(Error {
                            kind: ErrorKind::OperationAbortedByUser,
                        })
                    }
                    _ => {
                        return Err(Error {
                            kind: ErrorKind::InvalidOption,
                        })
                    }
                }
            }
        }
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::ErrorReadingFS,
            })
        }
    }
    Ok(())
}
fn create_dir(path: &str) -> Result<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::ErrorReadingFS,
        }),
    }
}
