use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    path: PathBuf,
}

fn cat(path: &Path) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(&path).expect("Failed to read file");
    println!("{}", file);
    Ok(())
}

fn main() {
    let args = Args::parse();
    cat(&args.path).unwrap();
}
