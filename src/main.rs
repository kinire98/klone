use std::{
    env,
    fmt::format,
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
    #[arg(short, long, name = "The directory of the files to backup")]
    origin_dir: Option<String>,
    /// The directory where you want to store the backup
    #[arg(
        short,
        long,
        name = "The directory where the backup is going to be stored"
    )]
    target_dir: Option<String>,
    /// Indicates if a new directory should be created
    #[arg(short = 'n', long)]
    new: bool,
    /// Add an exclusion so the files that match with it won't be backed up. Be cautious: this
    /// exclusions are global
    #[arg(short = 'e', long)]
    exclude: bool,
    /// Show all the exclusions you added
    #[arg(short = 'l', long)]
    list_exclusions: bool,
    /// Remove a previously added exclusion.
    #[arg(short = 'r', long)]
    remove_exclusion: bool,
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
    //env::set_var("COLORBT_SHOW_HIDDEN", "1");
    println!("{:?}", args);
    // ! This is gonna change when added defaults
    match (
        args.exclude,
        args.list_exclusions,
        args.remove_exclusion,
        args.origin_dir.is_some(), // If this call returns false,
        args.target_dir.is_some(), // it's imposible that this returns true
    ) {
        // Start backup
        (false, false, false, true, true) => backup_option(args)?,
        // Start backup without valid paths provided
        (false, false, false, _, _) => Err(Error { kind: ErrorKind::InvalidOption("You must specify two valid paths if you don't provide arguments for exclusions of defaults".to_string()) })?,
        // Add exclusion
        (true, false, false, false, false) => klone::config::exclusions::add_exclusion()?,
        // Add exclusion with conflicting arguments
        (true, _, _, _, _) => Err(Error { kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()) })?,
        // List exclusions
        (false, true, false, false, false) => klone::config::exclusions::list_exclusions()?,
        // List exclusions with conflicting arguments
        (_, true, _, _, _) => Err(Error { kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()) })?,
        // Delete exclusion
        (false, false, true, false, false) => klone::config::exclusions::remove_exclusion()?,
        // Delete exclusion with conflicting arguments
        (_, _, true, _, _) => Err(Error { kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()) })?,
    }
    Ok(())
}

fn backup_option(args: Args) -> Result<()> {
    let origin_dir = args.origin_dir.expect("This should not panic");
    let origin_dir = if origin_dir == ".".to_string() {
        std::env::current_dir().map_err(|_| Error {
            kind: ErrorKind::UndefinedError,
        })?
    } else {
        PathBuf::from(origin_dir)
    };
    let target_dir = args.target_dir.expect("This should not panic");
    let target_dir = if target_dir == ".".to_string() {
        std::env::current_dir().map_err(|_| Error {
            kind: ErrorKind::UndefinedError,
        })?
    } else {
        PathBuf::from(target_dir)
    };
    match origin_dir.try_exists() {
        Ok(exists) => {
            if !exists {
                let path = origin_dir.display().to_string();
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
    match target_dir.try_exists() {
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
            (true, false) => create_dir(target_dir.as_path())?,
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
                        create_dir(target_dir.as_path())?;
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
    klone::backup(origin_dir, target_dir)?;
    Ok(())
}

fn create_dir<T>(path: T) -> Result<()>
where
    T: AsRef<std::path::Path>,
{
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::FSError,
        }),
    }
}
