use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The directory where the files to backup are
    #[arg(short, long)]
    origin_dir: PathBuf,
    /// The directory where you want to store the backup
    #[arg(short, long)]
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
fn main() {}
