#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn find_exe(name: &str) -> Option<PathBuf> {
    if let Ok(paths) = std::env::var("PATH") {
        for path in std::env::split_paths(&paths) {
            let exe_path = path.join(name);
            if exe_path.is_file() {
                return Some(exe_path);
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let path_env = std::env::var("PATH").unwrap();
    
    print!("$ ");
    io::stdout().flush().unwrap();
    while stdin.read_line(&mut input).is_ok() {
        let trimmed_input = input.trim();
        let mut parts = trimmed_input.splitn(2, ' ');
        let first_word = parts.next().unwrap_or("");
        let remaining_text = parts.next().unwrap_or("");
        
        match first_word.to_lowercase().as_str() {
            "exit" => match remaining_text.to_lowercase().as_str(){
                "0" => break,
                _ => println!("{}: command not found", input.trim())
            },
            "type" => match remaining_text.to_lowercase().as_str(){
                "exit" => println!("{} is a shell builtin", remaining_text.trim()),
                "echo" => println!("{} is a shell builtin", remaining_text.trim()),
                "pwd" => println!("{} is a shell builtin", remaining_text.trim()),
                "cd" => println!("{} is a shell builtin", remaining_text.trim()),
                "type" => println!("{} is a shell builtin", remaining_text.trim()),
                _ => {
                    let mut split = path_env.split(':');
                    if let Some(path) = split.find(|path| std::fs::metadata(format!("{}/{}", path, remaining_text)).is_ok()) {
                        println!("{} is {}/{}", remaining_text.trim(), path, remaining_text.trim());
                    } else {
                        println!("{}: not found", remaining_text.trim());
                    }
                }
            },
            "pwd" => {
                let current_dir = std::env::current_dir().unwrap();
                println!("{}", current_dir.display());
            },
            "cd" => {
                if remaining_text.trim() == "~"{
                    let home = std::env::var("HOME").unwrap();
                    let path = Path::new(&home);
                    if let Err(_) = std::env::set_current_dir(&path) {
                        eprintln!("{}: No such file or directory", path.display());
                    }
                }else{
                    if std::env::set_current_dir(Path::new(remaining_text.trim())).is_err() {
                        println!("cd: {}: No such file or directory", remaining_text.trim());
                    }
                }
            },
            "echo" => println!("{}", remaining_text.trim()),
            _ => {
                if let Some(path) = find_exe(first_word.to_lowercase().as_str()) {
                    let parts = remaining_text.split_whitespace();
                    Command::new(path)
                        .args(parts)
                        .status()
                        .expect("failed to execute process");
                } else {
                    println!("{}: command not found", first_word.to_lowercase().as_str());
                }
                
            }
        };
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    
}
