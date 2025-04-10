# cargo-bins

A command-line tool to list your installed Cargo binaries. I found myself sometimes forgetting all the binaries that are installed in my `~/.cargo/bin` directory so I made this to help me quickly find them.

## Installation

| Source | Command |
| ------ | ------- |
| [crates.io](https://crates.io/crates/cargo-bins) | `cargo install cargo-bins` or `cargo install cargo-bins --no-default-features` for no colored output |
| [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) | `cargo binstall cargo-bins` |
| [GitHub](https://github.com/DarkCeptor44/cargo-bins) | `cargo install --git https://github.com/DarkCeptor44/cargo-bins.git` or `cargo install --git https://github.com/DarkCeptor44/cargo-bins.git --no-default-features` for no colored output |

## Usage

```sh
$ cargo bins -h
Lists all installed Cargo binaries

Usage: cargo bins [OPTIONS]

Options:
      --json     JSON output
      --reverse  Reverse sort
  -h, --help     Print help
  -V, --version  Print version
```

## Example

```sh
$ cargo bins
Binaries in ~/.cargo/bin:
  bat.exe
  cargo-audit.exe
  cargo-bins.exe
  cargo-binstall.exe
  cargo-cache.exe
  cargo-clippy.exe
```

## License

This project is licensed under the [GNU General Public License v3](LICENSE).
