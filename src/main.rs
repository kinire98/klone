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
    #[arg(short, long, name = "The directory of the files to backup")]
    origin_dir: Option<PathBuf>,
    /// The directory where you want to store the backup
    #[arg(
        short,
        long,
        name = "The directory where the backup is going to be stored"
    )]
    target_dir: Option<PathBuf>,
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
    /// Add, change or remove a default path.
    /// It will prompt if it's the target or the origin
    /// Leave it empty, to delete it
    #[arg(short, long, name = "The directory for the default")]
    defaults: Option<PathBuf>,
    /// It will show the default paths
    #[arg(short, long)]
    show_defaults: bool,
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
    // ! This is gonna change when added defaults
    match (
        args.exclude,
        args.list_exclusions,
        args.remove_exclusion,
        args.show_defaults,
        args.defaults.is_some(),
        args.origin_dir.is_some(), // If this call returns false,
        args.target_dir.is_some(), // it's imposible that this returns true
    ) {
        // Start backup
        // Start backup without valid paths provided
        (false, false, false, false, false, _, _) => backup_option(args)?,
        // Add exclusion
        (true, false, false, false, false, false, false) => {
            klone::config::exclusions::add_exclusion()?
        }
        // Add exclusion with conflicting arguments
        (true, _, _, _, _, _, _) => Err(Error {
            kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()),
        })?,
        // List exclusions
        (false, true, false, false, false, false, false) => {
            klone::config::exclusions::list_exclusions()?
        }
        // List exclusions with conflicting arguments
        (_, true, _, _, _, _, _) => Err(Error {
            kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()),
        })?,
        // Delete exclusion
        (false, false, true, false, false, false, false) => {
            klone::config::exclusions::remove_exclusion()?
        }
        // Delete exclusion with conflicting arguments
        (_, _, true, _, _, _, _) => Err(Error {
            kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()),
        })?,
        (false, false, false, false, true, false, false) => klone::config::defaults::set_defaults(
            args.defaults.expect("Already checked for it to exist"),
        )?,
        (_, _, _, _, true, _, _) => Err(Error {
            kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()),
        })?,
        (false, false, false, true, false, false, false) => {
            klone::config::defaults::print_defaults()?
        }
        (_, _, _, true, _, _, _) => Err(Error {
            kind: ErrorKind::InvalidOption("Conflicting arguments".to_string()),
        })?,
    }
    Ok(())
}

fn backup_option(args: Args) -> Result<()> {
    let origin_dir = if args.origin_dir.is_some() {
        args.origin_dir
            .expect("Won't panic, already checked for it to exist")
    } else {
        klone::config::defaults::get_default_origin()?
    };
    let target_dir = if args.target_dir.is_some() {
        args.target_dir
            .expect("Won't panic, already checked for it to exist")
    } else {
        klone::config::defaults::get_default_target()?
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
    // Checks if the target directory is inside the origin one.
    // This is forbidden because it will create infinite recursion
    let origin_dir = origin_dir.canonicalize().unwrap();
    let target_dir = target_dir.canonicalize().unwrap();
    // Takes ancestors of the target directory and checks them against
    // its ancestors. If one of them is equal to the origin directory
    // it returns an error
    if target_dir
        .ancestors()
        .any(|ancestor| ancestor == origin_dir.as_path())
    {
        Err(Error {
            kind: ErrorKind::TargetDirInsideOrigin,
        })?;
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
