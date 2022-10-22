use crate::Config;
use log::debug;
use std::process::Command;

pub fn save_to_disk(config: &Config) {
    debug!("Saving from {} to {}", config.log_dir, config.sync_dir);
    sync(&config.log_dir, &config.sync_dir);
}

pub fn load_from_disk(config: &Config) {
    debug!("Saving from {} to {}", config.sync_dir, config.log_dir);
    sync(&config.sync_dir, &config.log_dir)
}

fn sync(source: &str, destination: &str) {
    Command::new("cp")
        .arg("-rfpu")
        .arg(format!("{}/", source))
        .arg("-T")
        .arg(format!("{}/", destination))
        .output()
        .unwrap();
}
