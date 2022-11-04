# ram-journal
Efficiently in-memory log manager

Ram journal is a system that considerably reduces disk read and write operations by keeping logs from the /var/log directory in memory. It works in two ways:
### Sync mode(default)

In this mode, logs are kept in memory and are synced to disk every 24 hours. The sync interval can be changed
by  setting the `sync_interval` in `/etc/ram-journal/ram-journal.conf`:
```ini
# Sync to disk every 12 hours.  
sync_interval=12
```

/etc/ram-journal/ram-journal.conf

### Ephemeral mode(default)

In this mode, logs are kept only in memory and are lost after a system reboot.

To disable the sync, set `sync_interval` to `0` in `/etc/ram-journal/ram-journal.conf`:
```ini
# Disable sync.
sync_interval=0
```

## Installation

### APT Repository

#### Debian
```bash
sudo wget -q "https://neutrine.com/deb/public.gpg" -O /etc/apt/trusted.gpg.d/neutrine.com.gpg
sudo sh -c 'echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/trusted.gpg.d/neutrine.com.gpg] https://deb.neutrine.com bullseye main" > /etc/apt/sources.list.d/neutrine.com.list'
sudo apt update
sudo apt install ram-journal
# reboot the system
```

### Manually
#### x86_64
```bash
curl -fOL https://github.com/lpicanco/ram-journal/releases/download/v0.1.2/ram-journal-0.1.2-x86_64-unknown-linux-gnu.deb
sudo dpkg -i ram-journal-0.1.2-x86_64-unknown-linux-gnu.deb
# reboot the system
```

#### Raspberry 2-4(32 bits)
```bash
curl -fOL https://github.com/lpicanco/ram-journal/releases/download/v0.1.2/ram-journal-0.1.2-armv7-unknown-linux-gnueabihf.deb
sudo dpkg -i ram-journal-0.1.2-armv7-unknown-linux-gnueabihf.deb
# reboot the system
```

#### Raspberry 4(64 bits)
```bash
curl -fOL https://github.com/lpicanco/ram-journal/releases/download/v0.1.2/ram-journal-0.1.2-aarch64-unknown-linux-gnu.deb
sudo dpkg -i ram-journal-0.1.2-aarch64-unknown-linux-gnu.deb
# reboot the system
```


## Configuration
Configuration are kept in the `/etc/ram-journal/ram-journal.conf` file:
```ìni
# Log max size in megabytes.
#max_size=50

# Sync interval in hours. Zero to disable sync.
#sync_interval=24

# Directory sync the log.
#sync_dir=/var/lib/ram-journal/log

# Directory where the logs are saved.
#log_dir=/var/log

# Temporary device type.
#device=tmpfs
```
