//! Abstraction for the config specific to lantmäteriet's files.

use std::collections::HashMap;
use std::fs::File;
use std::io;

use serde::Deserialize;
use std::io::Read;
use std::path::Path;
use vadeen_osm::Tag;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub files: Vec<FileConfig>,
}

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub name: String,
    pub description: String,
    pub kkods: HashMap<String, Kkod>,
}

#[derive(Debug, Deserialize)]
pub struct Kkod {
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    pub files: Vec<FileConfig>,
    pub render: RenderConfig,
}

type RenderConfig = HashMap<String, Option<Vec<String>>>;

impl Config {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Config> {
        let mut file = File::open(path)?;
        let mut config = String::new();

        file.read_to_string(&mut config)?;
        Self::parse_string(&config)
    }

    pub fn parse_string(data: &str) -> io::Result<Config> {
        let ConfigFile { render, files } = serde_yaml::from_str(data).unwrap();

        let files: Vec<FileConfig> = files
            .into_iter()
            .filter(|f| render.contains_key(&f.name))
            .map(|c| Self::filter_kkods(c, &render))
            .collect();

        Ok(Config { files })
    }

    fn filter_kkods(file_config: FileConfig, render: &RenderConfig) -> FileConfig {
        let kkod_render_config = render.get(&file_config.name);
        if kkod_render_config.is_none() || kkod_render_config.unwrap().is_none() {
            return file_config;
        }

        let FileConfig {
            kkods,
            description,
            name,
        } = file_config;

        let enabled_kkods = kkod_render_config.unwrap().as_ref().unwrap();
        let kkods = kkods
            .into_iter()
            .filter(|(k, _)| enabled_kkods.contains(k))
            .collect();
        FileConfig {
            kkods,
            name,
            description,
        }
    }
}

impl Kkod {
    pub fn tags(&self) -> Vec<Tag> {
        self.tags
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()).into())
            .collect()
    }
}
