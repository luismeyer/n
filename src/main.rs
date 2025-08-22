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
    let mut current_dir = dir.to_path_buf();
    
    // Check current directory and up to 5 parent directories
    for _ in 0..=5 {
        if let Some(manager) = check_directory_for_package_manager(&current_dir) {
            return Some(manager);
        }
        
        // Try to move to parent directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            // Reached root directory, can't go up further
            break;
        }
    }
    
    None
}

fn check_directory_for_package_manager(dir: &std::path::Path) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
    
        if path.is_file() {
            if path.ends_with("package-lock.json") {
                return Some("npm".to_string());
            }
            
            if path.ends_with("yarn.lock") {
                return Some("yarn".to_string());
            }

            if path.ends_with("bun.lockb") || path.ends_with("bun.lock") {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_npm() {
        let temp_dir = TempDir::new().unwrap();
        let lock_file = temp_dir.path().join("package-lock.json");
        fs::write(&lock_file, "{}").unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        assert_eq!(result, Some("npm".to_string()));
    }

    #[test]
    fn test_detect_yarn() {
        let temp_dir = TempDir::new().unwrap();
        let lock_file = temp_dir.path().join("yarn.lock");
        fs::write(&lock_file, "").unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        assert_eq!(result, Some("yarn".to_string()));
    }

    #[test]
    fn test_detect_bun() {
        let temp_dir = TempDir::new().unwrap();
        let lock_file = temp_dir.path().join("bun.lockb");
        fs::write(&lock_file, "").unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        assert_eq!(result, Some("bun".to_string()));
    }

    #[test]
    fn test_detect_pnpm() {
        let temp_dir = TempDir::new().unwrap();
        let lock_file = temp_dir.path().join("pnpm-lock.yaml");
        fs::write(&lock_file, "").unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        assert_eq!(result, Some("pnpm".to_string()));
    }

    #[test]
    fn test_no_package_manager() {
        let temp_dir = TempDir::new().unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_in_parent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let parent_path = temp_dir.path();
        let child_path = parent_path.join("child");
        fs::create_dir(&child_path).unwrap();
        
        // Create lock file in parent directory
        let lock_file = parent_path.join("package-lock.json");
        fs::write(&lock_file, "{}").unwrap();
        
        // Test detection from child directory
        let result = detect_package_manager(&child_path);
        assert_eq!(result, Some("npm".to_string()));
    }

    #[test]
    fn test_detect_in_grandparent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let root_path = temp_dir.path();
        let child_path = root_path.join("child");
        let grandchild_path = child_path.join("grandchild");
        
        fs::create_dir_all(&grandchild_path).unwrap();
        
        // Create lock file in root directory
        let lock_file = root_path.join("yarn.lock");
        fs::write(&lock_file, "").unwrap();
        
        // Test detection from grandchild directory
        let result = detect_package_manager(&grandchild_path);
        assert_eq!(result, Some("yarn".to_string()));
    }

    #[test]
    fn test_multiple_package_managers_npm_priority() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create multiple lock files
        fs::write(temp_dir.path().join("package-lock.json"), "{}").unwrap();
        fs::write(temp_dir.path().join("yarn.lock"), "").unwrap();
        fs::write(temp_dir.path().join("bun.lockb"), "").unwrap();
        
        let result = detect_package_manager(temp_dir.path());
        // The order depends on filesystem iteration order, but should return one of them
        assert!(result.is_some());
        let manager = result.unwrap();
        assert!(manager == "npm" || manager == "yarn" || manager == "bun");
    }

    #[test]
    fn test_max_directory_traversal() {
        let temp_dir = TempDir::new().unwrap();
        let mut current_path = temp_dir.path().to_path_buf();
        
        // Create a deep directory structure (more than 5 levels)
        for i in 0..8 {
            current_path = current_path.join(format!("level{}", i));
            fs::create_dir_all(&current_path).unwrap();
        }
        
        // Create lock file in the temp root (more than 5 levels up)
        let lock_file = temp_dir.path().join("package-lock.json");
        fs::write(&lock_file, "{}").unwrap();
        
        // Test detection from deep directory - should NOT find it (exceeds 5 parent limit)
        let result = detect_package_manager(&current_path);
        assert_eq!(result, None);
    }

    #[test]
    fn test_within_traversal_limit() {
        let temp_dir = TempDir::new().unwrap();
        let mut current_path = temp_dir.path().to_path_buf();
        
        // Create a directory structure within the 5-level limit
        for i in 0..4 {
            current_path = current_path.join(format!("level{}", i));
            fs::create_dir_all(&current_path).unwrap();
        }
        
        // Create lock file in the temp root (4 levels up, within limit)
        let lock_file = temp_dir.path().join("bun.lockb");
        fs::write(&lock_file, "").unwrap();
        
        // Test detection from deep directory - should find it
        let result = detect_package_manager(&current_path);
        assert_eq!(result, Some("bun".to_string()));
    }
}
