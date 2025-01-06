use std::fs;
use std::process::Command;

const MEMINFO_FILE: &str = "/proc/meminfo";

enum MemoryKind {
    RamTotal,
    RamFree,
    SwapTotal,
    SwapFree,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct RAMInfo {
    pub total: i64,
    pub free: i64,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct SwapInfo {
    pub total: i64,
    pub free: i64,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct DiskInfo {
    pub root_total: i64,
    pub root_free: i64,
    pub root_used: i64,
    pub home_total: Option<i64>,
    pub home_free: Option<i64>,
    pub home_used: Option<i64>,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct SystemMemory {
    pub ram: RAMInfo,
    pub swap: SwapInfo,
    pub disk: DiskInfo,
}

impl SystemMemory {
    pub fn new() -> SystemMemory {
        SystemMemory {
            ram: RAMInfo {
                total: get_meminfo_file(MemoryKind::RamTotal).unwrap_or(0),
                free: get_meminfo_file(MemoryKind::RamFree).unwrap_or(0),
            },
            swap: SwapInfo {
                total: get_meminfo_file(MemoryKind::SwapTotal).unwrap_or(0),
                free: get_meminfo_file(MemoryKind::SwapFree).unwrap_or(0),
            },
            disk: get_disk_info().unwrap_or_default(),
        }
    }
}

fn get_meminfo_file(kind: MemoryKind) -> anyhow::Result<i64> {
    let containable = match kind {
        MemoryKind::RamTotal => "MemTotal",
        MemoryKind::RamFree => "MemFree",
        MemoryKind::SwapTotal => "SwapTotal",
        MemoryKind::SwapFree => "SwapFree",
    };

    let memfile: String = fs::read_to_string(MEMINFO_FILE)?;
    let mem_total_line = memfile
        .lines()
        .filter(|l| l.contains(containable))
        .collect::<String>();
    let sublines: Vec<&str> = mem_total_line.split_whitespace().collect();
    let mem = sublines[1].parse::<i64>()? / 1024;
    Ok(mem)
}

fn get_disk_info() -> anyhow::Result<DiskInfo> {
    let output = Command::new("df").output()?;
    let output = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output.split('\n').collect();

    let mut disk_info = DiskInfo::default();

    for line in lines {
        let splits: Vec<&str> = line.split_whitespace().collect();
        if let Some(last) = splits.last() {
            match *last {
                "/" => {
                    disk_info.root_total = splits[1].parse::<i64>()? / 1024;
                    disk_info.root_used = splits[2].parse::<i64>()? / 1024;
                    disk_info.root_free = splits[3].parse::<i64>()? / 1024;
                }
                "/home" => {
                    disk_info.home_total = Some(splits[1].parse::<i64>()? / 1024);
                    disk_info.home_used = Some(splits[2].parse::<i64>()? / 1024);
                    disk_info.home_free = Some(splits[3].parse::<i64>()? / 1024);
                }
                _ => {}
            }
        }
    }
    Ok(disk_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_home_partition() -> bool {
        let output = Command::new("df").output().unwrap();
        let output = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output.split('\n').collect();
        for line in lines {
            let splits: Vec<&str> = line.split_whitespace().collect();
            if let Some(last) = splits.last() {
                if *last == "/home" {
                    return true;
                }
            }
        }
        false
    }

    #[test]
    fn test_get_meminfo_file() {
        let ramtotal = get_meminfo_file(MemoryKind::RamTotal).unwrap();
        let ramfree = get_meminfo_file(MemoryKind::RamFree).unwrap();
        let swaptotal = get_meminfo_file(MemoryKind::SwapTotal).unwrap();
        let swapfree = get_meminfo_file(MemoryKind::SwapTotal).unwrap();
        assert!(ramtotal >= 0);
        assert!(ramfree >= 0);
        assert!(swaptotal >= 0);
        assert!(swapfree >= 0);
    }

    #[test]
    fn test_disk_info_defaults() {
        let disk_info = DiskInfo::default();
        assert_eq!(disk_info.root_total, 0);
        assert_eq!(disk_info.root_free, 0);
        assert_eq!(disk_info.root_used, 0);
        assert_eq!(disk_info.home_total, None);
        assert_eq!(disk_info.home_free, None);
        assert_eq!(disk_info.home_used, None);
    }

    #[test]
    fn test_get_disk_info() {
        let disk_info = get_disk_info().unwrap();
        assert!(disk_info.root_total >= 0);
        assert!(disk_info.root_used >= 0);
        assert!(disk_info.root_free >= 0);

        if check_home_partition() {
            assert!(disk_info.home_total >= Some(0));
            assert!(disk_info.home_used >= Some(0));
            assert!(disk_info.home_free >= Some(0));
        } else {
            assert!(disk_info.home_total.is_none());
            assert!(disk_info.home_used.is_none());
            assert!(disk_info.home_free.is_none());
        }
    }

    #[test]
    fn test_system_memory_defaults() {
        let system_memory = SystemMemory::default();

        assert_eq!(system_memory.ram.total, 0);
        assert_eq!(system_memory.ram.free, 0);

        assert_eq!(system_memory.swap.total, 0);
        assert_eq!(system_memory.swap.free, 0);

        assert_eq!(system_memory.disk.root_total, 0);
        assert_eq!(system_memory.disk.root_free, 0);
        assert_eq!(system_memory.disk.root_used, 0);
        assert_eq!(system_memory.disk.home_total, None);
        assert_eq!(system_memory.disk.home_free, None);
        assert_eq!(system_memory.disk.home_used, None);
    }

    #[test]
    fn test_system_memory() {
        let system_memory = SystemMemory::new();

        assert!(system_memory.ram.total >= 0);
        assert!(system_memory.ram.free >= 0);

        assert!(system_memory.swap.total >= 0);
        assert!(system_memory.swap.free >= 0);

        assert!(system_memory.disk.root_total >= 0);
        assert!(system_memory.disk.root_free >= 0);
        assert!(system_memory.disk.root_used >= 0);

        if check_home_partition() {
            assert!(system_memory.disk.home_total >= Some(0));
            assert!(system_memory.disk.home_used >= Some(0));
            assert!(system_memory.disk.home_free >= Some(0));
        } else {
            assert!(system_memory.disk.home_total.is_none());
            assert!(system_memory.disk.home_used.is_none());
            assert!(system_memory.disk.home_free.is_none());
        }
    }
}
