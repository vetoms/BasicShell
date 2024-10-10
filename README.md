# Simple Rust Shell

This project is a simple command-line shell implemented in Rust, supporting basic commands and file system navigation. It includes commands like `pwd`, `cd`, `echo`, `clear`, `dir`, and `type`, as well as the ability to execute system commands located in the `PATH`.

## Features

- **`pwd`**: Prints the current working directory.
- **`cd`**: Changes the current directory. Supports `..` for navigating to the parent directory and `~` for the home directory.
- **`dir`**: Lists the files in the current directory, showing file sizes and modification timestamps.
- **`echo`**: Prints the specified text to the shell.
- **`clear`**: Clears the terminal screen.
- **`type`**: Checks if a command is a shell builtin or a system command located in the `PATH`.

## Prerequisites

To run this shell program, you need:
- Rust (version 1.40 or newer)
- A Unix-like environment (Linux, macOS) or Windows

## Installation

1. Clone this repository to your local machine:
    ```bash
    git clone https://github.com/vetoms/BasicShell.git
    ```

2. Navigate to the project directory:
    ```bash
    cd simple_rust_shell
    ```

3. Build the project using Cargo:
    ```bash
    cargo build --release
    ```

4. Run the shell:
    ```bash
    ./target/release/simple_rust_shell
    ```

## Usage

Once the shell is running, you can execute commands as you would in a standard terminal:

- **Navigation**: Use `cd` to change directories.
- **Directory Listing**: Use `dir` to list files and folders in the current directory.
- **Current Directory**: Use `pwd` to print the current directory path.
- **Text Output**: Use `echo <text>` to print text to the terminal.
- **Clear Screen**: Use `clear` to clear the terminal screen.
- **Command Check**: Use `type <command>` to check if a command is a shell builtin or an executable in the `PATH`.

### Example

```bash
$ pwd
/home/user/simple_rust_shell

$ cd ..
$ dir
Name: example.txt
Size: 123 bytes
Modified: 2023-05-10 14:32:23
---

$ echo Hello, Rust Shell!
Hello, Rust Shell!

$ type echo
echo is a shell builtin

$ clear
