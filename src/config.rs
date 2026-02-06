use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_editor")]
    pub editor: String,

    #[serde(default = "default_editor_args")]
    pub editor_args: Vec<String>,

    #[serde(default = "default_show_hidden")]
    pub show_hidden: bool,

    #[serde(default = "default_preview_max_lines")]
    pub preview_max_lines: usize,

    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_editor() -> String {
    "vim".to_string()
}

fn default_editor_args() -> Vec<String> {
    vec![]
}

fn default_show_hidden() -> bool {
    false
}

fn default_preview_max_lines() -> usize {
    1000
}

fn default_theme() -> String {
    "base16-ocean.dark".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: default_editor(),
            editor_args: default_editor_args(),
            show_hidden: default_show_hidden(),
            preview_max_lines: default_preview_max_lines(),
            theme: default_theme(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        eprintln!("Failed to parse config: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to read config: {}", e);
                }
            }
        }

        Self::default()
    }

    pub fn config_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "vive-file-viewer") {
            let config_dir = proj_dirs.config_dir();
            config_dir.join("config.toml")
        } else {
            PathBuf::from("~/.config/vive-file-viewer/config.toml")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.editor, "vim");
        assert!(config.editor_args.is_empty());
        assert!(!config.show_hidden);
        assert_eq!(config.preview_max_lines, 1000);
        assert_eq!(config.theme, "base16-ocean.dark");
    }

    #[test]
    fn test_parse_config_from_toml() {
        let toml_str = r#"
            editor = "nvim"
            editor_args = ["-c", "startinsert"]
            show_hidden = true
            preview_max_lines = 500
            theme = "Solarized (dark)"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.editor, "nvim");
        assert_eq!(config.editor_args, vec!["-c", "startinsert"]);
        assert!(config.show_hidden);
        assert_eq!(config.preview_max_lines, 500);
        assert_eq!(config.theme, "Solarized (dark)");
    }

    #[test]
    fn test_parse_partial_config() {
        let toml_str = r#"
            editor = "code"
        "#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.editor, "code");
        // Other fields should have defaults
        assert!(!config.show_hidden);
        assert_eq!(config.preview_max_lines, 1000);
    }

    #[test]
    fn test_config_path_is_not_empty() {
        let path = Config::config_path();
        assert!(!path.as_os_str().is_empty());
    }
}
