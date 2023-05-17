# File-Editor-CLI

## Description
This project can be categorized as a mini-shell service for editing files. The functionalities of the program allow users to read files, write to files, find instances of words, create files, and remove files.

Also, note that the documentation below is primarily made for Linux-like OS.

## Installing `rustup`
For Unix-like OS: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

For Windows download and run: https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe

## Usage

In order to use the program, simply download/clone the code into your preferred text editor and use the terminal:

First we will need to change directory into our program: `cd cli`

To access the shell: `cargo run`

| Command | Description |
| --- | --- |
| R `<filename>` |reads the contents of a file|
| W `<filename>` |writes to a file (will create the file if it doesn't exist or append to a file if it does exist)|
| C `<filename>` |creates a file|
| find `<filename>` `<query>`|finds all instances of query word in file|
| size `<filename>` |finds the size of a file|
| rm `<filename>` |removes/deletes a file|
| oW `<filename>` |overwrites contents of a file with user defined location|
| man |presents the manual|
| findAcr `<filename>` `<query>` |finds a word across multiple files given an input file|
| pwd |print working directory|

Note that **`rustup`** needs to be installed on the device before these commands can be executed.

