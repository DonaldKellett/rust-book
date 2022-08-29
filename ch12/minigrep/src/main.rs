use clap::Parser;
use envy;
use serde::Deserialize;
use std::{error::Error, fs, process};

fn main() {
    let args = Args::parse();
    let env = envy::from_env::<Env>().unwrap_or(Env { ignore_case: None });

    if let Err(e) = run(args, env) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/// A miniature version of grep
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Substring to query for
    #[clap(value_parser)]
    query: String,

    /// Path to file with contents to query over
    #[clap(value_parser)]
    file_path: String,

    /// Perform a case-insensitive query
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    ignore_case: bool,
}

#[derive(Deserialize)]
struct Env {
    ignore_case: Option<bool>,
}

fn run(args: Args, env: Env) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.file_path)?;

    let results = minigrep::search(
        &args.query,
        &contents,
        args.ignore_case || env.ignore_case.unwrap_or(false),
    );

    if results.len() == 0 {
        process::exit(1);
    }

    for line in results {
        println!("{line}");
    }

    Ok(())
}
