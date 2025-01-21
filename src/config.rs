use std::path::{Path, PathBuf};

use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    /// The directory from which new images will be sourced for import.
    pub source: PathBuf,

    /// The directory to which imported images will be moved.
    pub target: PathBuf,

    /// The format used to name files when they are imported.
    pub template: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            source: PathBuf::from("ingestion"),
            target: PathBuf::from("photos"),
            template: "{year}/{date_time}-{name}".to_string(),
        }
    }
}

impl Config {
    pub fn load<S: AsRef<Path>>(source: S) -> Result<Config, crate::errors::Error> {
        let content = std::fs::read_to_string(source).map_err(|e|
            crate::errors::user_with_internal(
                "Failed to read your configuration file.",
                "Make sure that the file exists and you have permission to access it.",
                e))?;

        toml::from_str(&content)
            .map_err(|e| crate::errors::user_with_internal(
                "Failed to parse your configuration file.",
                "Make sure that your configuration file is valid TOML and matches the configuration schema.",
                e))
    }
}