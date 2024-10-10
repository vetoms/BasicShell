#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use std::os::windows::fs::MetadataExt; // Needed on Windows for the file size
use std::time::{SystemTime, UNIX_EPOCH};

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

fn format_unix_timestamp(timestamp: u64) -> String {
    let datetime = UNIX_EPOCH + std::time::Duration::new(timestamp, 0);
    let datetime: chrono::DateTime<chrono::Local> = datetime.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    let path_env = std::env::var("PATH").unwrap();
    let mut current_dir = std::env::current_dir().unwrap();

    print!("{}>$ ",current_dir.display());
    io::stdout().flush().unwrap();
    while stdin.read_line(&mut input).is_ok() {
        let trimmed_input = input.trim();
        let mut parts = trimmed_input.splitn(2, ' ');
        let first_word = parts.next().unwrap_or("");
        let remaining_text = parts.next().unwrap_or("");
        
        match first_word.to_lowercase().as_str() {
            "exit" => break,
            "type" => match remaining_text.to_lowercase().as_str(){
                "exit" => println!("{} is a shell builtin", remaining_text.trim()),
                "echo" => println!("{} is a shell builtin", remaining_text.trim()),
                "pwd" => println!("{} is a shell builtin", remaining_text.trim()),
                "cd" => println!("{} is a shell builtin", remaining_text.trim()),
                "dir" => println!("{} is a shell builtin", remaining_text.trim()),
                "clear" => println!("{} is a shell builtin", remaining_text.trim()),
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
            "clear" => {
                clear_screen();
            }
            "dir" => {                
                match fs::read_dir(std::env::current_dir().unwrap()) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(entry) => {
                                    let path = entry.path();
                                    let metadata = fs::metadata(&path).expect("Unable to get metadata");
                                    
                                    // Get the file size if it's a file
                                    let size = if metadata.is_file() {
                                        Some(metadata.len())
                                    } else {
                                        None
                                    };
            
                                    // Get the modified time
                                    let modified_time = metadata.modified().expect("Unable to get modification time");
                                    let duration = modified_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
                                    let modified_secs = duration.as_secs();
                                    
                                    println!(
                                        "Name: {}\nSize: {} bytes\nModified: {}",
                                        path.display(),
                                        size.map_or("N/A".to_string(), |s| s.to_string()),
                                        format_unix_timestamp(modified_secs)
                                    );
                                    println!("---");
                                }
                                Err(e) => eprintln!("Error reading entry: {}", e),
                            }
                        }
                    }
                    Err(e) => eprintln!("Error reading directory: {}", e),
                }
            },
            "pwd" => {                
                println!("{}", current_dir.display());
            },
            "cd" => {
                if remaining_text.trim() == "~"{
                    let home = std::env::var("HOME").unwrap();
                    let path = Path::new(&home);
                    if let Err(_) = std::env::set_current_dir(&path) {
                        eprintln!("{}: No such file or directory", path.display());
                    }else{
                        current_dir = std::env::current_dir().unwrap();
                    }
                
                }else if remaining_text.trim() == ".."{
                    if let Some(parent_path) = current_dir.parent() {
                        std::env::set_current_dir(parent_path).expect("Failed to change to parent directory");
                
                        let new_path = std::env::current_dir().expect("Failed to get current directory");
                        current_dir = std::env::current_dir().unwrap();
                    } else {
                        println!("There is no parent directory.");
                    }
                }else{
                    if std::env::set_current_dir(Path::new(remaining_text.trim())).is_err() {
                        println!("cd: {}: No such file or directory", remaining_text.trim());
                    }else{
                        current_dir = std::env::current_dir().unwrap();
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
        print!("{}>$ ",current_dir.display());
        io::stdout().flush().unwrap();
    }

    
}
