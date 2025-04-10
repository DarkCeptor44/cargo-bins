#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod types;

use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use dirs::home_dir;
use serde_json::json;
use std::{fs::read_dir, path::Path, process::exit};
use types::Binary;

#[cfg(feature = "color")]
use colored::Colorize;

const NAME: &str = "cargo-bins:";

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Lists all installed Cargo binaries")]
    Bins(BinsArgs),
}

#[derive(Parser)]
struct BinsArgs {
    #[arg(long, help = "JSON output", default_value_t = false)]
    json: bool,

    #[arg(long, help = "Reverse sort", default_value_t = false)]
    reverse: bool,
}

fn main() {
    if let Err(e) = run() {
        #[cfg(feature = "color")]
        eprintln!("{} {}", NAME.red().bold(), e.to_string().red());

        #[cfg(not(feature = "color"))]
        eprintln!("{NAME} {e}");

        exit(1);
    }
}

fn run() -> Result<()> {
    let args = App::parse();

    match args.command {
        Command::Bins(bins_args) => {
            let home_dir = home_dir().ok_or(anyhow!("failed to get home directory"))?;
            let mut files = cargo_binaries(&home_dir)?;

            if bins_args.json {
                println!("{}", json!(files));
                return Ok(());
            }

            if bins_args.reverse {
                files.sort_by(|a, b| b.name.to_lowercase().cmp(&a.name.to_lowercase()));
            } else {
                files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            }

            let msg = format!("Binaries in {}:", home_dir.display());

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
        }
    }

    Ok(())
}

/// Returns a list of binaries in the `~/.cargo/bin` directory
///
/// ## Arguments
///
/// * `path` - The path to retrieve binaries from
///
/// ## Returns
///
/// `Vec<Binary>` - A list of [binaries](Binary)
///
/// ## Errors
///
/// Returns an error if failed to read directory `path`
fn cargo_binaries(path: &Path) -> Result<Vec<Binary>> {
    let cargo_path = path.join(".cargo").join("bin");

    if !cargo_path.is_dir() {
        return Ok(Vec::new());
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
    use std::{
        fs::{File, create_dir_all, remove_dir_all},
        io::Write,
    };
    use tempfile::tempdir;

    const N: usize = 30;

    #[test]
    fn test_cargo_binaries() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path();
        let tmp_path = path.join(".cargo").join("bin");

        create_dir_all(&tmp_path)?;

        for i in 0..N {
            let mut file = File::create_new(tmp_path.join(format!("file-{i}")))?;
            file.write_all("test".as_bytes())?;
            drop(file);
        }

        let binaries = cargo_binaries(path)?;

        remove_dir_all(path)?;

        assert_eq!(binaries.len(), N);
        println!("Found {} binaries.", binaries.len());
        Ok(())
    }
}
