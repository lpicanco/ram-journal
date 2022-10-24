# ram-journal
Efficiently in-memory log manager

Ram journal is a system that considerably reduces disk read and write operations by keeping logs from the /var/log directory in memory. It works in two ways:

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
