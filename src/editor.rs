use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::config::Config;

pub struct Editor {
    command: String,
    args: Vec<String>,
}

impl Editor {
    pub fn new(config: &Config) -> Self {
        Self {
            command: config.editor.clone(),
            args: config.editor_args.clone(),
        }
    }

    /// Validate the file path before opening in editor
    fn validate_path(path: &Path) -> Result<(), String> {
        // Check if path exists
        if !path.exists() {
            return Err(format!("File does not exist: {}", path.display()));
        }

        // Resolve symlinks and check the target exists
        match path.canonicalize() {
            Ok(real_path) => {
                if !real_path.exists() {
                    return Err(format!(
                        "Symlink target does not exist: {}",
                        real_path.display()
                    ));
                }
            }
            Err(e) => {
                return Err(format!("Failed to resolve path: {}", e));
            }
        }

        Ok(())
    }

    pub fn open(&self, path: &Path) -> Result<(), String> {
        // Validate path before opening
        Self::validate_path(path)?;

        let path_str = path.to_string_lossy().to_string();

        // Restore terminal to normal state
        disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {}", e))?;
        execute!(io::stdout(), LeaveAlternateScreen)
            .map_err(|e| format!("Failed to leave alternate screen: {}", e))?;

        // Run editor with inherited stdio
        let mut cmd = Command::new(&self.command);
        for arg in &self.args {
            cmd.arg(arg);
        }
        cmd.arg(&path_str);
        cmd.stdin(Stdio::inherit());
        cmd.stdout(Stdio::inherit());
        cmd.stderr(Stdio::inherit());

        let result = match cmd.spawn() {
            Ok(mut child) => match child.wait() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Editor process error: {}", e)),
            },
            Err(e) => Err(format!("Failed to open editor '{}': {}", self.command, e)),
        };

        // Restore TUI state
        enable_raw_mode().map_err(|e| format!("Failed to enable raw mode: {}", e))?;
        execute!(io::stdout(), EnterAlternateScreen)
            .map_err(|e| format!("Failed to enter alternate screen: {}", e))?;

        // Force redraw
        io::stdout().flush().ok();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_validate_path_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let result = Editor::validate_path(&file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_nonexistent_file() {
        let result = Editor::validate_path(Path::new("/nonexistent/path/to/file.txt"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_validate_path_existing_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = Editor::validate_path(temp_dir.path());
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_validate_path_valid_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("original.txt");
        let link_path = temp_dir.path().join("link.txt");

        File::create(&file_path).unwrap();
        symlink(&file_path, &link_path).unwrap();

        let result = Editor::validate_path(&link_path);
        assert!(result.is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn test_validate_path_broken_symlink() {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new().unwrap();
        let link_path = temp_dir.path().join("broken_link.txt");

        // Create symlink to nonexistent target
        symlink("/nonexistent/target", &link_path).unwrap();

        let result = Editor::validate_path(&link_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_editor_new() {
        let config = Config {
            editor: "nvim".to_string(),
            editor_args: vec!["-c".to_string(), "startinsert".to_string()],
            ..Config::default()
        };
        let editor = Editor::new(&config);
        assert_eq!(editor.command, "nvim");
        assert_eq!(editor.args, vec!["-c", "startinsert"]);
    }
}
