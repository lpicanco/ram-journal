use crate::Config;
use lms_lib::{core::synchronize, parse::Flag};
use log::debug;

pub fn save_to_disk(config: &Config) {
    debug!("Saving from {} to {}", config.log_dir, config.sync_dir);
    synchronize(&config.log_dir, &config.sync_dir, Flag::empty()).unwrap();
}

pub fn load_from_disk(config: &Config) {
    debug!("Saving from {} to {}", config.sync_dir, config.log_dir);
    synchronize(&config.sync_dir, &config.log_dir, Flag::empty()).unwrap();
}
