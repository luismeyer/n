use std::env;
use std::fs;
use std::process::Command as ProcessCommand;
use std::path::Path;
use dialoguer::Select;
use dialoguer::console::style;
use serde_json::Value;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn main() {
    // Collecting all arguments except for the first one (which is the program name)
    let args: Vec<String> = env::args().skip(1).collect();
    
    let current_dir = env::current_dir().expect("Failed to read current directory");

    match detect_package_manager(&current_dir) {
        Some(manager) => run_command(&manager, &args),
        None => handle_no_package_manager(&args),
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
    let current_dir = env::current_dir().expect("Failed to read current directory");
    let patched_args = patch_commands(manager, args, &current_dir);
    
    let status = ProcessCommand::new(manager)
        .args(patched_args)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed to execute");
    }
}

fn patch_commands(manager: &str, args: &[String], dir: &Path) -> Vec<String> {
    if args.is_empty() {
        return args.to_vec();
    }

    let mut result = Vec::new();
    
    // Get the first argument (the command to potentially patch)
    let first_arg = &args[0];
    
    // First try autocorrect for script commands
    let corrected_command = try_autocorrect_script(manager, first_arg, dir);
    
    // Apply command patching based on the package manager
    let patched_command = match manager {
        "npm" => patch_npm_command(&corrected_command),
        "yarn" => patch_yarn_command(&corrected_command),
        "pnpm" => patch_pnpm_command(&corrected_command),
        "bun" => patch_bun_command(&corrected_command),
        _ => vec![corrected_command],
    };
    
    // Add the patched command(s)
    result.extend(patched_command);
    
    // Add the remaining arguments
    if args.len() > 1 {
        result.extend_from_slice(&args[1..]);
    }
    
    result
}

fn try_autocorrect_script(manager: &str, cmd: &str, dir: &Path) -> String {
    // Skip autocorrect for known package manager commands
    let known_commands = match manager {
        "npm" => vec!["install", "i", "uninstall", "r", "rm", "start", "s", "test", "t", 
                     "update", "up", "list", "ls", "init", "publish", "pack", "version", "audit"],
        "yarn" => vec!["install", "i", "add", "a", "remove", "rm", "start", "s", "test", "t", 
                      "upgrade", "up", "list", "ls", "init", "publish", "pack", "version", "audit"],
        "pnpm" => vec!["install", "i", "add", "a", "remove", "rm", "start", "s", "test", "t", 
                      "update", "up", "list", "ls", "init", "publish", "pack", "version", "audit"],
        "bun" => vec!["install", "i", "add", "a", "remove", "rm", "start", "s", "test", "t", 
                     "update", "up", "list", "ls", "init", "publish", "pack", "version", "audit"],
        _ => vec![],
    };
    
    // If it's a known package manager command, don't try autocorrect
    if known_commands.contains(&cmd) {
        return cmd.to_string();
    }
    
    // Try autocorrect for potential script commands
    autocorrect_command(cmd, dir)
}

fn patch_npm_command(cmd: &str) -> Vec<String> {
    match cmd {
        "i" => vec!["install".to_string()],
        "a" => vec!["install".to_string()], // npm doesn't have 'add', use 'install'
        "r" => vec!["uninstall".to_string()],
        "rm" => vec!["uninstall".to_string()],
        "d" => vec!["run".to_string(), "dev".to_string()],
        "dev" => vec!["run".to_string(), "dev".to_string()],
        "b" => vec!["run".to_string(), "build".to_string()],
        "build" => vec!["run".to_string(), "build".to_string()],
        "s" => vec!["start".to_string()],
        "t" => vec!["test".to_string()],
        "up" => vec!["update".to_string()],
        "ls" => vec!["list".to_string()],
        _ => vec![cmd.to_string()],
    }
}

fn patch_yarn_command(cmd: &str) -> Vec<String> {
    match cmd {
        "i" => vec!["install".to_string()],
        "a" => vec!["add".to_string()],
        "r" => vec!["remove".to_string()],
        "rm" => vec!["remove".to_string()],
        "d" => vec!["dev".to_string()],
        "b" => vec!["build".to_string()],
        "s" => vec!["start".to_string()],
        "t" => vec!["test".to_string()],
        "up" => vec!["upgrade".to_string()],
        "ls" => vec!["list".to_string()],
        _ => vec![cmd.to_string()],
    }
}

fn patch_pnpm_command(cmd: &str) -> Vec<String> {
    match cmd {
        "i" => vec!["install".to_string()],
        "a" => vec!["add".to_string()],
        "r" => vec!["remove".to_string()],
        "rm" => vec!["remove".to_string()],
        "d" => vec!["run".to_string(), "dev".to_string()],
        "dev" => vec!["run".to_string(), "dev".to_string()],
        "b" => vec!["run".to_string(), "build".to_string()],
        "build" => vec!["run".to_string(), "build".to_string()],
        "s" => vec!["start".to_string()],
        "t" => vec!["test".to_string()],
        "up" => vec!["update".to_string()],
        "ls" => vec!["list".to_string()],
        _ => vec![cmd.to_string()],
    }
}

fn patch_bun_command(cmd: &str) -> Vec<String> {
    match cmd {
        "i" => vec!["install".to_string()],
        "a" => vec!["add".to_string()],
        "r" => vec!["remove".to_string()],
        "rm" => vec!["remove".to_string()],
        "d" => vec!["run".to_string(), "dev".to_string()],
        "dev" => vec!["run".to_string(), "dev".to_string()],
        "b" => vec!["run".to_string(), "build".to_string()],
        "build" => vec!["run".to_string(), "build".to_string()],
        "s" => vec!["start".to_string()],
        "t" => vec!["test".to_string()],
        "up" => vec!["update".to_string()],
        "ls" => vec!["list".to_string()],
        _ => vec![cmd.to_string()],
    }
}

fn handle_no_package_manager(args: &[String]) {
    let options = vec!["pnpm","bun","npm", "yarn"];
    
    let selection = Select::new()
        .with_prompt("No package manager detected. Please select one:")
        .items(&options)
        .default(0)
        .interact()
        .expect("Failed to get selection");
    
    let manager = options[selection];
    println!("Selected: {}", manager);

    // Check if the original command is already an install command
    if is_install_command(args) {
        // If it's already an install command, just run it once
        println!("Running install command to initialize project and install packages...");
        run_command(manager, args);
    } else {
        // If it's not an install command, first initialize with install, then run the original command
        println!("Initializing project with {}...", manager);
        let init_args = vec!["install".to_string()];
        run_command(manager, &init_args);
        
        if !args.is_empty() {
            println!("Running original command...");
            run_command(manager, args);
        }
    }
}

fn is_install_command(args: &[String]) -> bool {
    if args.is_empty() {
        return false;
    }
    
    let first_arg = &args[0];
    
    // Check if it's an install command or shorthand that maps to install
    matches!(first_arg.as_str(), "install" | "i" | "add" | "a")
}

fn read_package_json_scripts(dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut current_dir = dir.to_path_buf();
    
    // Look for package.json in current directory and up to 5 parent directories
    for _ in 0..=5 {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.exists() {
            let content = fs::read_to_string(&package_json_path)?;
            let json: Value = serde_json::from_str(&content)?;
            
            if let Some(scripts) = json.get("scripts") {
                if let Some(scripts_obj) = scripts.as_object() {
                    return Ok(scripts_obj.keys().cloned().collect());
                }
            }
            return Ok(Vec::new());
        }
        
        // Try to move to parent directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent.to_path_buf();
        } else {
            break;
        }
    }
    
    Ok(Vec::new())
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

fn find_similar_command(input: &str, available_commands: &[String]) -> Option<(String, i64)> {
    let matcher = SkimMatcherV2::default();
    let mut best_match = None;
    let mut best_score = 0i64;
    
    // Normalize input by removing common separators
    let normalized_input = input.replace('-', "").replace('_', "").to_lowercase();
    
    for command in available_commands {
        // Try multiple matching strategies
        let normalized_command = command.replace('-', "").replace('_', "").to_lowercase();
        
        // Calculate fuzzy match scores
        let scores = [
            // Original order: fuzzy_match(command, input) - command as haystack
            matcher.fuzzy_match(command, input),
            // Reversed order: fuzzy_match(input, command) - input as haystack
            matcher.fuzzy_match(input, command),
            // Normalized versions
            matcher.fuzzy_match(&normalized_command, &normalized_input),
            matcher.fuzzy_match(&normalized_input, &normalized_command),
            matcher.fuzzy_match(&normalized_input, command),
            matcher.fuzzy_match(input, &normalized_command),
        ];
        
        // Find best fuzzy score
        for score_opt in scores.iter() {
            if let Some(score) = score_opt {
                if *score > best_score {
                    best_score = *score;
                    best_match = Some((command.clone(), *score));
                }
            }
        }
        
        // Fallback to edit distance for close matches
        if best_match.is_none() || best_score < 50 {
            let edit_dist = levenshtein_distance(&normalized_input, &normalized_command);
            let max_len = std::cmp::max(normalized_input.len(), normalized_command.len());
            
            // Convert edit distance to a similarity score (higher is better)
            if max_len > 0 {
                let similarity_ratio = 1.0 - (edit_dist as f64 / max_len as f64);
                // Convert to score similar to fuzzy match (scale by 100)
                let edit_score = (similarity_ratio * 100.0) as i64;
                
                // Use edit distance score if it's better and meets minimum similarity
                // Be more conservative - only use for reasonably similar strings
                if edit_score > best_score && similarity_ratio > 0.75 && edit_dist <= 3 {
                    best_score = edit_score;
                    best_match = Some((command.clone(), edit_score));
                }
            }
        }
    }
    
    // Use a higher threshold to avoid false positives
    if best_score > 60 {
        best_match
    } else {
        None
    }
}

fn autocorrect_command(cmd: &str, dir: &Path) -> String {
    // First check if we can get scripts from package.json
    if let Ok(scripts) = read_package_json_scripts(dir) {
        if !scripts.is_empty() {
            // Check if command exists exactly
            if scripts.contains(&cmd.to_string()) {
                return cmd.to_string();
            }
            
            // Try to find a similar command
            if let Some((suggested, _score)) = find_similar_command(cmd, &scripts) {
                // Log the correction with colored output for visibility
                eprintln!(
                    "{}",
                    style(format!(
                        "✓ Autocorrected '{}' → '{}'", 
                        cmd, 
                        suggested
                    )).yellow().bold()
                );
                return suggested;
            }
        }
    }
    
    // Return original command if no correction found or user declined
    cmd.to_string()
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

    #[test]
    fn test_patch_commands_npm_install() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["i".to_string(), "lodash".to_string()];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, vec!["install", "lodash"]);
    }

    #[test]
    fn test_patch_commands_npm_run_dev() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["d".to_string()];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, vec!["run", "dev"]);
    }

    #[test]
    fn test_patch_commands_yarn_add() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["a".to_string(), "react".to_string()];
        let result = patch_commands("yarn", &args, temp_dir.path());
        assert_eq!(result, vec!["add", "react"]);
    }

    #[test]
    fn test_patch_commands_pnpm_remove() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["r".to_string(), "lodash".to_string()];
        let result = patch_commands("pnpm", &args, temp_dir.path());
        assert_eq!(result, vec!["remove", "lodash"]);
    }

    #[test]
    fn test_patch_commands_bun_build() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["b".to_string()];
        let result = patch_commands("bun", &args, temp_dir.path());
        assert_eq!(result, vec!["run", "build"]);
    }

    #[test]
    fn test_patch_commands_no_patching_needed() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["install".to_string(), "lodash".to_string()];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, vec!["install", "lodash"]);
    }

    #[test]
    fn test_patch_commands_unknown_manager() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["i".to_string(), "lodash".to_string()];
        let result = patch_commands("unknown", &args, temp_dir.path());
        assert_eq!(result, vec!["i", "lodash"]);
    }

    #[test]
    fn test_patch_commands_empty_args() {
        let temp_dir = TempDir::new().unwrap();
        let args: Vec<String> = vec![];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_patch_commands_preserves_additional_args() {
        let temp_dir = TempDir::new().unwrap();
        let args = vec!["i".to_string(), "lodash".to_string(), "--save-dev".to_string()];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, vec!["install", "lodash", "--save-dev"]);
    }

    #[test]
    fn test_patch_npm_vs_yarn_add_command() {
        let temp_dir = TempDir::new().unwrap();
        // npm doesn't have 'add', should map 'a' to 'install'
        let npm_result = patch_commands("npm", &vec!["a".to_string(), "lodash".to_string()], temp_dir.path());
        assert_eq!(npm_result, vec!["install", "lodash"]);
        
        // yarn has 'add', should map 'a' to 'add'
        let yarn_result = patch_commands("yarn", &vec!["a".to_string(), "lodash".to_string()], temp_dir.path());
        assert_eq!(yarn_result, vec!["add", "lodash"]);
    }

    #[test]
    fn test_patch_commands_yarn_dev_vs_npm_run_dev() {
        let temp_dir = TempDir::new().unwrap();
        // yarn can run dev directly
        let yarn_result = patch_commands("yarn", &vec!["d".to_string()], temp_dir.path());
        assert_eq!(yarn_result, vec!["dev"]);
        
        // npm needs 'run dev'
        let npm_result = patch_commands("npm", &vec!["d".to_string()], temp_dir.path());
        assert_eq!(npm_result, vec!["run", "dev"]);
    }

    #[test]
    fn test_patch_commands_all_shortcuts() {
        let shortcuts = vec![
            ("i", vec!["install"]),
            ("r", vec!["uninstall"]),
            ("rm", vec!["uninstall"]),
            ("s", vec!["start"]),
            ("t", vec!["test"]),
            ("up", vec!["update"]),
            ("ls", vec!["list"]),
        ];

        let temp_dir = TempDir::new().unwrap();
        for (shortcut, expected) in shortcuts {
            let result = patch_commands("npm", &vec![shortcut.to_string()], temp_dir.path());
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_is_install_command_true_cases() {
        assert!(is_install_command(&vec!["install".to_string()]));
        assert!(is_install_command(&vec!["i".to_string()]));
        assert!(is_install_command(&vec!["add".to_string()]));
        assert!(is_install_command(&vec!["a".to_string()]));
        assert!(is_install_command(&vec!["i".to_string(), "lodash".to_string()]));
        assert!(is_install_command(&vec!["add".to_string(), "react".to_string(), "--save-dev".to_string()]));
    }

    #[test]
    fn test_is_install_command_false_cases() {
        assert!(!is_install_command(&vec![]));
        assert!(!is_install_command(&vec!["build".to_string()]));
        assert!(!is_install_command(&vec!["dev".to_string()]));
        assert!(!is_install_command(&vec!["test".to_string()]));
        assert!(!is_install_command(&vec!["start".to_string()]));
        assert!(!is_install_command(&vec!["remove".to_string(), "lodash".to_string()]));
        assert!(!is_install_command(&vec!["uninstall".to_string(), "react".to_string()]));
    }

    #[test]
    fn test_is_install_command_with_shortcuts() {
        // Test that shortcuts that map to install commands are detected
        assert!(is_install_command(&vec!["i".to_string()])); // maps to install
        assert!(is_install_command(&vec!["a".to_string()])); // maps to add/install
        
        // Test that other shortcuts are not detected as install commands
        assert!(!is_install_command(&vec!["r".to_string()])); // maps to remove/uninstall
        assert!(!is_install_command(&vec!["d".to_string()])); // maps to dev
        assert!(!is_install_command(&vec!["b".to_string()])); // maps to build
    }

    #[test]
    fn test_read_package_json_scripts_success() {
        let temp_dir = TempDir::new().unwrap();
        let package_json_content = r#"
        {
            "name": "test-project",
            "scripts": {
                "dev": "next dev",
                "build": "next build",
                "start": "next start",
                "typecheck": "tsc --noEmit",
                "test": "jest"
            }
        }
        "#;
        
        let package_json_path = temp_dir.path().join("package.json");
        fs::write(&package_json_path, package_json_content).unwrap();
        
        let scripts = read_package_json_scripts(temp_dir.path()).unwrap();
        let mut expected = vec!["dev".to_string(), "build".to_string(), "start".to_string(), 
                               "typecheck".to_string(), "test".to_string()];
        let mut actual = scripts.clone();
        expected.sort();
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_read_package_json_scripts_no_scripts() {
        let temp_dir = TempDir::new().unwrap();
        let package_json_content = r#"
        {
            "name": "test-project",
            "dependencies": {
                "react": "^18.0.0"
            }
        }
        "#;
        
        let package_json_path = temp_dir.path().join("package.json");
        fs::write(&package_json_path, package_json_content).unwrap();
        
        let scripts = read_package_json_scripts(temp_dir.path()).unwrap();
        assert_eq!(scripts, Vec::<String>::new());
    }

    #[test]
    fn test_read_package_json_scripts_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let scripts = read_package_json_scripts(temp_dir.path()).unwrap();
        assert_eq!(scripts, Vec::<String>::new());
    }

    #[test]
    fn test_find_similar_command() {
        let commands = vec![
            "dev".to_string(),
            "build".to_string(),
            "typecheck".to_string(),
            "test".to_string(),
            "lint".to_string(),
        ];
        
        // Test exact match should score high
        let result = find_similar_command("dev", &commands);
        assert!(result.is_some());
        let (matched, score) = result.unwrap();
        assert_eq!(matched, "dev");
        assert!(score > 50);
        
        // Test user's specific case: type-check -> typecheck
        let result = find_similar_command("type-check", &commands);
        assert!(result.is_some(), "type-check should match typecheck");
        let (matched, _score) = result.unwrap();
        assert_eq!(matched, "typecheck");
        
        // Test user's second case: typechock -> typecheck
        let result = find_similar_command("typechock", &commands);
        assert!(result.is_some(), "typechock should match typecheck");
        let (matched, _score) = result.unwrap();
        assert_eq!(matched, "typecheck");
        
        // Test closer match that should work
        let result = find_similar_command("typechek", &commands);
        assert!(result.is_some());
        let (matched, _score) = result.unwrap();
        assert_eq!(matched, "typecheck");
        
        // Test no good match
        let result = find_similar_command("completely-different", &commands);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_autocorrect_script_known_commands() {
        let temp_dir = TempDir::new().unwrap();
        
        // Known npm commands should not be autocorrected
        assert_eq!(try_autocorrect_script("npm", "install", temp_dir.path()), "install");
        assert_eq!(try_autocorrect_script("npm", "i", temp_dir.path()), "i");
        assert_eq!(try_autocorrect_script("yarn", "add", temp_dir.path()), "add");
        assert_eq!(try_autocorrect_script("pnpm", "remove", temp_dir.path()), "remove");
    }

    #[test]
    fn test_autocorrect_command_with_scripts() {
        let temp_dir = TempDir::new().unwrap();
        let package_json_content = r#"
        {
            "name": "test-project",
            "scripts": {
                "typecheck": "tsc --noEmit"
            }
        }
        "#;
        
        let package_json_path = temp_dir.path().join("package.json");
        fs::write(&package_json_path, package_json_content).unwrap();
        
        // Test exact match
        let result = autocorrect_command("typecheck", temp_dir.path());
        assert_eq!(result, "typecheck");
    }

    #[test]
    fn test_patch_commands_with_autocorrect() {
        let temp_dir = TempDir::new().unwrap();
        let package_json_content = r#"
        {
            "name": "test-project",
            "scripts": {
                "typecheck": "tsc --noEmit",
                "dev": "next dev"
            }
        }
        "#;
        
        let package_json_path = temp_dir.path().join("package.json");
        fs::write(&package_json_path, package_json_content).unwrap();
        
        // Test that existing script doesn't get modified by npm's run prefix
        let args = vec!["typecheck".to_string()];
        let result = patch_commands("npm", &args, temp_dir.path());
        assert_eq!(result, vec!["typecheck"]);
    }
}
