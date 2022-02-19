# wordle-finder
Rust program to help find the wordle word

## Use
This program can either be used from a terminal, `./wordle_finder --help` for more info, or with the GUI frontend, launched by running `./wordle_finder -g` in a terminal

<details>
  <summary>GUI Frontend for version 2.0.0</summary>
![GUI](https://user-images.githubusercontent.com/58576759/154787824-4358b931-f161-4ecb-9859-ba21066512c2.png)
</details>

## Downloading
I provide pre-built binaries on the releases page. I do not test the Windows binary, but it may work.

## Building
First, install rust using your package manager or from https://www.rust-lang.org/. After you have rust installed, download or clone the repository and then run `cargo build --release` in the repository folder. The binary should then be found at `./target/release/wordle_finder` or `./target/release/wordle_finder.exe` if you're on Windows.
