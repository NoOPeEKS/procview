# Processes information
Processes information can be found in /proc/[pid]/stat, /proc/[pid]/cmdline, /proc/[pid]/status.

# CPU Cores usage
CPU Cores usage can be found in /proc/stat

# System RAM, Disk and Swap
For RAM and swap, /proc/meminfo. For disk usage, parse outputs of command 'df'.

# Hardware and System Info
For CPU and Kernel, /proc/cpuinfo and /proc/version. For GPUs, /proc/driver/nvidia/, or /sys/class/drm/. For uptime, /proc/uptime. For the terminal command was called on /proc/self/fd/0.
```rust
fn get_tty() -> Option<String> {
    if let Ok(tty) = fs::read_link("/proc/self/fd/0") {
        return tty.to_str().map(String::from);
    }
    None
}
```
