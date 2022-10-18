use clap::Parser;

/// Efficiently in-memory log manager
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    /// Log max size in megabytes.
    #[arg(long, default_value_t = 50)]
    pub max_size: u32,

    /// Sync interval in hours. Zero to disable sync.
    #[arg(long, default_value_t = 24)]
    pub sync_interval: u32,

    /// Directory sync the log.
    #[arg(long, default_value = "/var/lib/ram-journal/log")]
    pub sync_dir: String,

    /// Directory where the logs are saved
    #[arg(long, default_value = "/var/log")]
    pub log_dir: String,

    /// Filesystem
    #[arg(long, default_value = "tmpfs")]
    pub device: String,
}
