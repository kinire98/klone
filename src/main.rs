use std::{
    env, fs,
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
    /// Indicates if a new directory should be created
    #[arg(short, long)]
    new: bool,
}

// 1. Check if the paths are valid.
// 2. Check if you want to create a new path. If so check that the path does not exist and create
//    it.
// 3. Give to the library files only two valid path which it can work with directly.
// 4. Return the correspondent errors if neccesary.
fn main() -> Result<()> {
    let args = Args::parse();
    color_eyre::install().unwrap();
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("COLORBT_SHOW_HIDDEN", "1");
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
                kind: ErrorKind::FSError,
            })
        }
    }
    match args.target_dir.try_exists() {
        Ok(exists) => match (args.new, exists) {
            // The target directory exists and it is told to create it.
            // Aborts because it can be the case that there is a backup or other data in that
            // location
            (true, true) => {
                return Err(Error {
                    kind: ErrorKind::InvalidOption("This directory already exists!".to_string()),
                })
            }
            // The directory doesn't exist and the program is told to create it
            (true, false) => create_dir(args.target_dir.to_str().unwrap())?,
            // The directory doesn't exist and the user hasn't asked to create it
            // It prompts if the user wants to create it.
            // If so, and there aren't any errors the execution continues as normal
            (false, false) => {
                print!("The target directory does not exist. Do you want to create it? [Y/n]:  ");
                let _ = io::stdout().flush();
                let mut input = String::new();
                io::stdin()
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
                            kind: ErrorKind::InvalidOption(format!(
                                "The input you introduced ({}) is not valid. Only y or n",
                                input
                            )),
                        })
                    }
                }
            }
            // Normal execution, the directory exists and it isn't asked to create it
            (false, true) => (),
        },
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::FSError,
            })
        }
    }

    // In this point we have two directories we know for a fact that exist
    klone::backup(args.origin_dir, args.target_dir)?;
    Ok(())
}

fn create_dir(path: &str) -> Result<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::FSError,
        }),
    }
}
