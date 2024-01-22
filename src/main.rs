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

fn main() {}
