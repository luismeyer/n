use std::env;
use std::fs;
use std::process::Command as ProcessCommand;

fn main() {
    // Collecting all arguments except for the first one (which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();
    
    let current_dir = env::current_dir().expect("Failed to read current directory");

    match detect_package_manager(&current_dir) {
        Some(manager) => run_command(&manager, &args),
        None => println!("No package manager detected."),
    }
}

fn detect_package_manager(dir: &std::path::Path) -> Option<String> {
    let entries = fs::read_dir(dir).expect("Failed to read directory entries");
    
    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
    
    if path.is_file() {
            if path.ends_with("package-lock.json") {
                return Some("npm".to_string());
            }
            
            if path.ends_with("yarn.lock") {
                return Some("yarn".to_string());
            }

            if path.ends_with("bun.lockb") {
                return Some("bun".to_string());
            }

            if path.ends_with("pnpm-lock.yaml") {
                return Some("pnpm".to_string());
            }
        }
    }
    None
}

fn run_command(manager: &str, args: &[String]) {
    let status = ProcessCommand::new(manager)
        .args(args)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed to execute");
    }
}
