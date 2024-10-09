use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Deserialize)]
struct Rules {
    mapping: HashMap<String, Vec<String>>,
    ignore: Vec<String>,
}

pub struct Config {
    pub working_dir: PathBuf,
    pub mapping: HashMap<String, String>,
    pub ignored: Vec<String>,
}

impl Config {
    pub fn new(working_dir: &str) -> Result<Self, io::Error> {
        let path = PathBuf::from(working_dir);

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Directory {} does not exist", path.display()),
            ));
        }

        let content = fs::read_to_string("rules.toml")?;
        let rules: Rules = toml::from_str(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        let mut mapping: HashMap<String, String> = HashMap::new();
        for (key, exts) in rules.mapping {
            for ext in exts {
                let formatted_ext = if ext.starts_with('.') {
                    ext
                } else {
                    format!(".{}", ext)
                };
                mapping.insert(formatted_ext, key.clone());
            }
        }

        Ok(Self {
            working_dir: path,
            mapping,
            ignored: rules.ignore,
        })
    }
}