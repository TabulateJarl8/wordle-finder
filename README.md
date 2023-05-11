![Wordle Finder](img/wordle_finder_logo_dark.png#gh-dark-mode-only)
![Wordle Finder](img/wordle_finder_logo_light.png#gh-light-mode-only)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/L4L3L7IO2)

<a href="https://aur.archlinux.org/packages/wordle-finder/"><img alt="AUR version" src="https://img.shields.io/aur/version/wordle-finder"></a>  

# Rust program to help find the wordle word

## Use
### From the terminal
Run `./wordle-finder --help` for more info.

### As a GUI
If you'd like to use the GUI frontend, you can launch it by running `./wordle-finder -g` in a terminal or opening the binary from a file explorer. You can type letters into their respective boxes and use the backspace key to delete them. In order to toggle yellow and dark grey letters, you can click on the appropriate key in the keyboard. 1 click changes it to dark grey, click it again to turn it to yellow, and click it a third time to change it back to it's original state. The reset key will reset all of your typed letters and keyboard keys, but keep the currently displayed wordlist. The enter key will submit the current state and return all possible words in the right-hand side column.

![GUI Example](img/main_ui.png)

## Downloading and Installation

I maintain an AUR package for Arch Linux users:

```
git clone https://aur.archlinux.org/wordle-finder.git && cd wordle-finder
makepkg -si
```

For anyone else, or if you'd like the portable version, I provide pre-built binaries on the releases page. I do not test the Windows binary, but it may work.

### Windows Users
If you launch the program by double clicking and it immediately closes, or you get an error similar to this: `thread 'main' panicked at 'Error when running GUI: WebView2Error(WindowsError(Error { code: 0x80070002, message: The system cannot find the file specified., win32_error: 2 }))'`, then you should try installing the WebView2 Runtime from the [Microsoft website](https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section).

## Building
First, install rust using your package manager or from https://www.rust-lang.org/.

#### Linux systems with make
Linux systems with make installed can make the project that way `make linux` or `make windows-x64` will make the respective binaries and output them in the `dist/` folder.

#### Other systems
Other systems can build the binary for their system by running `cargo build --release` in the repository folder. The resulting binary can be found at `./target/release/wordle-finder` or `./target/release/wordle-finder.exe` if you're on Windows.
