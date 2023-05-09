# File-Editor-CLI

## Installing `rustup`
For Unix-like OS: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

For Windows download and run: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

## Usage

In order to use the program, simply download/clone the code into your preferred text editor and use the terminal:

| Command | Description |
| --- | --- |
| cargo run R `<filename>` |reads the contents of a file|
| cargo run W `<filename>` |writes to a file (will create the file if it doesn't exist or append to a file if it does exist)|
| cargo run C `<filename>` |creates a file|
| cargo run find `<filename>` `<query>`|finds all instances of query word in file|
| cargo run size `<filename>` |finds the size of a file|
| cargo run rm `<filename>` |removes/deletes a file|
| cargo run oW `<filename>` |overwrites contents of a file with user defined location|

Note that **`rustup`** needs to be installed on the device before these commands can be executed.

