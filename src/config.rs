use std::{env, fs, io::{Error, ErrorKind}, path::Path};

#[derive(Debug, serde_derive::Deserialize)]
pub struct Config {
    pub data: String,
    pub common: String,
    pub log: bool,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let file;

        if let Ok(val) = env::var("XDG_CONFIG_HOME") {
            file = format!("{}/proton.conf", val);
        } else if let Ok(val) = env::var("HOME") {
            file = format!("{}/.config/proton.conf", val);
        } else {
            return Err(Error::new(ErrorKind::NotFound, "failed to find config directory (check environment)"));
        }

        if !Path::new(&file).exists() {
            return Err(Error::new(ErrorKind::NotFound, format!("{} does not exist", file)));
        }

        let config: Self = toml::from_str(&fs::read_to_string(file)?)?;

        Ok(config)
    }
}
