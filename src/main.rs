extern crate sys_mount;

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, thread};

use clap::Parser;
use clokwerk::{Scheduler, TimeUnits};
use signal_hook::flag;
use sys_mount::{FilesystemType, Mount, MountFlags, Unmount, UnmountFlags};

use crate::config::Config;

mod config;
mod sync;

fn main() {
    let config: Config = Config::parse();
    println!("Args: {:?}", config);

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
        thread::sleep(Duration::from_millis(1000));
    }

    println!("Exiting");
    if config.sync_interval > 0 {
        sync::save_to_disk(&config);
    }
    umount(mount);
}

fn umount(mount: Mount) {
    mount.unmount(UnmountFlags::empty()).unwrap()
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

    println!("Mounted");
    return mount;
}
