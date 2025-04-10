#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod types;

use anyhow::{Result, anyhow};
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
    fn run() -> Result<()> {
        let args = App::parse();
        let mut files = cargo_binaries()?;

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

/// Returns a list of binaries in the `~/.cargo/bin` directory
///
/// ## Returns
///
/// `Vec<Binary>` - A list of [binaries](Binary)
///
/// ## Errors
///
/// Returns an error if:
///
/// - Failed to get home directory
/// - `~/.cargo/bin` is not a directory
/// - Failed to read directory
pub(crate) fn cargo_binaries() -> Result<Vec<Binary>> {
    let home_dir = home_dir().ok_or(anyhow!("failed to get home directory"))?;
    let cargo_path = home_dir.join(".cargo").join("bin");

    if !cargo_path.is_dir() {
        return Err(anyhow!("`{}` is not a directory", cargo_path.display()));
    }

    let binaries = read_dir(cargo_path)?
        .filter_map(Result::ok)
        .map(Binary::from)
        .collect();

    Ok(binaries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_binaries() -> Result<()> {
        let binaries = cargo_binaries()?;
        assert!(!binaries.is_empty());
        Ok(())
    }
}
