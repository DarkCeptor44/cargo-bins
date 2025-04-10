#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod types;

use clap::Parser;
use dirs::home_dir;
use serde_json::json;
use std::{fs::read_dir, process::exit};
use types::Binary;

#[cfg(feature = "color")]
use colored::Colorize;

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
        #[cfg(feature = "color")]
        eprintln!("{} {}", "cargo-bins:".red().bold(), e.to_string().red());

        #[cfg(not(feature = "color"))]
        eprintln!("cargo-bins: {e}");

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
            files.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
        }

        let msg = "Binaries in ~/.cargo/bin:";

        #[cfg(feature = "color")]
        println!("{}", msg.green().bold());

        #[cfg(not(feature = "color"))]
        println!("{msg}");

        for file in &files {
            #[cfg(feature = "color")]
            println!("  {}", file.name.blue());

            #[cfg(not(feature = "color"))]
            println!("  {}", file.name);
        }

        Ok(())
    }
}
