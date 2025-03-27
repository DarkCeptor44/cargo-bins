#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod types;

use clap::Parser;
use colored::Colorize;
use dirs::home_dir;
use serde_json::json;
use std::{fs::read_dir, process::exit};
use types::Binary;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct App {
    #[arg(long, help = "JSON output", default_value_t = false)]
    json: bool,

    #[arg(long, help = "Reverse sort", default_value_t = false)]
    reverse: bool,
}

fn main() {
    if let Err(e) = App::run() {
        eprintln!("{} {}", "cargo-bins:".red().bold(), e.to_string().red());
        exit(1);
    }
}

impl App {
    fn run() -> Result<(), Box<dyn std::error::Error>> {
        let args = App::parse();
        let home_dir = home_dir().ok_or("Failed to get home directory")?;
        let cargo_path = home_dir.join(".cargo").join("bin");

        let mut files: Vec<Binary> = read_dir(cargo_path)?
            .filter_map(Result::ok)
            .map(|e| Binary {
                name: e.file_name().to_string_lossy().to_string(),
                path: e.path().display().to_string(),
            })
            .collect();

        if args.json {
            println!("{}", json!(files));
            return Ok(());
        }

        if args.reverse {
            files.sort_by(|a, b| b.name.cmp(&a.name));
        }

        println!("{}", "Binaries in ~/.cargo/bin:".green().bold());
        for file in &files {
            println!("  {file}");
        }

        Ok(())
    }
}
