use std::{env, fs};

const CPU_INFO_FILE: &str = "/proc/cpuinfo";
const KERNEL_INFO_FILE: &str = "/proc/version";
const UPTIME_FILE: &str = "/proc/uptime";

fn get_tty() -> Option<String> {
    env::var("TERM").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tty_name() {
        env::set_var("TERM", "tmux-256color");
        assert_eq!(get_tty(), Some("tmux-256color".to_string()));
    }
}
