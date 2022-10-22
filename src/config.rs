use clap::{CommandFactory, Parser};
use log::debug;
use std::path::Path;
use twelf::{config, Layer};

/// Efficiently in-memory log manager
#[config]
#[derive(Parser, Clone, Debug)]
#[clap(version, about, long_about = None)]
pub struct Config {
    /// Log max size in megabytes.
    #[clap(long, required = false)]
    #[serde(default = "defaults::max_size")]
    pub max_size: u32,

    /// Sync interval in hours. Zero to disable sync.
    #[clap(long, required = false)]
    #[serde(default = "defaults::sync_interval")]
    pub sync_interval: u32,

    /// Directory sync the log.
    #[clap(long, required = false)]
    #[serde(default = "defaults::sync_dir")]
    pub sync_dir: String,

    /// Directory where the logs are saved
    #[clap(long, required = false)]
    #[serde(default = "defaults::log_dir")]
    pub log_dir: String,

    /// Filesystem
    #[clap(long, required = false)]
    #[serde(default = "defaults::device")]
    pub device: String,

    /// Config file
    #[clap(long, required = false)]
    #[serde(default = "defaults::config_file")]
    pub config_file: String,
}

mod defaults {
    pub fn max_size() -> u32 {
        50
    }

    pub fn sync_interval() -> u32 {
        24
    }

    pub fn sync_dir() -> String {
        String::from("/var/lib/ram-journal/log")
    }

    pub fn log_dir() -> String {
        String::from("/var/log")
    }

    pub fn device() -> String {
        String::from("tmpfs")
    }

    pub fn config_file() -> String {
        String::from("/etc/ram-journal/ram-journal.conf")
    }
}

impl Config {
    pub fn load() -> Config {
        let matches = Config::command().get_matches();

        let mut layers: Vec<Layer> = Vec::new();

        if let Some(config_file) = matches.get_one::<String>("config-file") {
            layers.push(Layer::Ini(config_file.into()));
        } else if Path::new(&defaults::config_file()).is_file() {
            layers.push(Layer::Ini(defaults::config_file().into()));
        }

        layers.push(Layer::Env(Some(String::from("RAM_JOURNAL"))));
        layers.push(Layer::Clap(matches));

        let config = Config::with_layers(&layers).unwrap();
        debug!("configuration:  {:?}", config);
        return config;
    }
}
