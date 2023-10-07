use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use crate::matcher::SourceMatcher;
use crate::processor::SourceProcessor;

#[derive(Deserialize, Debug)]
pub struct Common {
    pub use_export_name: bool,
    pub use_sigscan: bool,
}

impl Default for Common {
    fn default() -> Self {
        Common {
            use_export_name: true,
            use_sigscan: true,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ConfigRule {
    pub matcher: Box<SourceMatcher>,
    pub processors: Vec<SourceProcessor>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub common: Common,
    pub rules: HashMap<String, ConfigRule>,
}

pub trait FileConfig {
    fn load_from_toml(file_path: &Path) -> Config;
}

impl FileConfig for Config {
    fn load_from_toml(file_path: &Path) -> Config {
        let mut file = File::open(file_path).expect("open config file failed");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("read config file failed");
        toml::from_str(file_content.as_str()).expect("parse config file failed")
    }
}
