# Custom Command-Line Shell in Rust

This project is a simple, custom command-line shell implemented in Rust. It can execute basic commands like `pwd`, `cd`, `echo`, and `type`. The shell also supports running external commands that can be found in the system's `PATH` environment variable.

## Features

- **Built-in Commands**:
  - `exit [0]`: Exits the shell.
  - `pwd`: Prints the current working directory.
  - `cd [directory]`: Changes the current directory to the specified one. If no directory is provided, it defaults to the user's home directory.
  - `echo [text]`: Prints the specified text to the console.
  - `type [command]`: Displays whether the specified command is a shell built-in or an external command.
  
- **External Commands**:
  - The shell can locate and execute external commands found in the `PATH` environment variable.
  
- **Input Handling**:
  - This shell continuously waits for user input, processes it, and executes the command. After each command, it prompts the user again until the `exit 0` command is used to terminate the shell.

## Code Structure

The code is divided into a few key sections:

- **`find_exe` function**: This function searches for an executable file with the specified name within the system's `PATH`. It returns the file path if the executable is found.

- **Main Execution**:
  - The `main` function sets up a loop to read input from `stdin`, then processes and executes the command. The loop will continue until the user types `exit 0` or forcefully exits the shell.
  
- **Command Matching**:
  - The code uses pattern matching to handle built-in commands such as `exit`, `type`, `pwd`, `cd`, and `echo`.
  - If the command isn't a built-in, the shell checks if it’s an executable in the system’s `PATH`.
  - For recognized executables, the command is executed using `Command::new` with any additional arguments passed by the user.

## Usage

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Building and Running

To build and run this project, follow these steps:

1. Clone the repository or copy the code into a new Rust project.

2. Open a terminal and navigate to the project directory.

3. Compile the code:
   ```bash
   cargo build --release
