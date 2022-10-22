extern crate sys_mount;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use clokwerk::{Scheduler, TimeUnits};
use signal_hook::flag;
use simple_logger::SimpleLogger;
use sys_mount::{FilesystemType, Mount, MountFlags, Unmount, UnmountFlags};

use crate::config::Config;

use log::{info, LevelFilter};

mod config;
mod sync;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_module_level("lms_lib", LevelFilter::Warn)
        .init()
        .unwrap();

    let config: Config = Config::load();

    // If sync dir doesn't exists(first run), save to disk.
    if config.sync_interval > 0 && !Path::new(&config.sync_dir).is_dir() {
        fs::create_dir_all(&config.sync_dir).unwrap();
        sync::save_to_disk(&config);
    }

    let mount = mount(&config);

    if config.sync_interval > 0 {
        sync::load_from_disk(&config);
    }

    let mut scheduler = schedule_sync(&config);

    let term = Arc::new(AtomicBool::new(false));
    flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
    flag::register(signal_hook::consts::SIGINT, Arc::clone(&term)).unwrap();

    while !term.load(Ordering::Relaxed) {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(1));
    }

    if config.sync_interval > 0 {
        sync::save_to_disk(&config);
    }
    umount(mount);
}

fn umount(mount: Mount) {
    mount.unmount(UnmountFlags::DETACH).unwrap();
    info!("{} Unmounted", mount.target_path().to_str().unwrap());
}

fn schedule_sync(config: &Config) -> Scheduler {
    let mut scheduler = Scheduler::new();

    if config.sync_interval > 0 {
        let conf = config.clone();
        scheduler
            .every(config.sync_interval.hours())
            .run(move || sync::save_to_disk(&conf));
    }
    return scheduler;
}

fn mount(config: &Config) -> Mount {
    let mut mount_builder = Mount::builder()
        .flags(MountFlags::empty())
        .fstype(FilesystemType::Manual(&config.device))
        .flags(MountFlags::NOATIME | MountFlags::NODEV | MountFlags::NOSUID);

    let options: String;
    if config.max_size > 0 {
        options = format!("size={}M", config.max_size);
        mount_builder = mount_builder.data(&options);
    }

    let mount = mount_builder.mount("ram-journal", &config.log_dir).unwrap();

    info!("{} Mounted", &config.log_dir);
    return mount;
}
