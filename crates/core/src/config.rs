use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use crate::identifier::Identifiers;
use crate::matcher::SourceMatcherEnum;
use crate::processor::SourceProcessor;

#[derive(Deserialize, Debug)]
pub(crate) struct ConfigRule {
    pub(crate) matcher: Box<SourceMatcherEnum>,
    pub(crate) processors: Vec<SourceProcessor>,
}

#[derive(Deserialize, Default, Debug)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) identifiers: Identifiers,
    pub(crate) rules: HashMap<String, ConfigRule>,
}

pub(crate) trait ReadFromFile {
    fn load_from_toml(file_path: &Path) -> Self;
}

impl ReadFromFile for Config {
    fn load_from_toml(file_path: &Path) -> Config {
        let mut file = File::open(file_path).expect("open config file failed");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("read config file failed");
        toml::from_str(file_content.as_str()).expect("parse config file failed")
    }
}
