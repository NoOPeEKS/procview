use std::fs;

const STAT_FILE: &str = "/proc/stat";

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CPUCore {
    pub name: String,
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct CPUInfo {
    pub cores: Vec<CPUCore>,
}

impl CPUInfo {
    pub fn new() -> CPUInfo {
        CPUInfo {
            cores: CPUInfo::get_cpu_usage().unwrap()
        }
    }

    fn get_cpu_usage() -> anyhow::Result<Vec<CPUCore>> {
        let cpu_usage_file = fs::read_to_string(STAT_FILE)?;
        let cores_strings: Vec<String> = cpu_usage_file
            .lines()
            .filter(|l| l.starts_with("cpu"))
            .map(|s| s.to_string())
            .collect();

        let mut cores: Vec<CPUCore> = Vec::new();
        for core in cores_strings {
            let mut cpu_core = CPUCore::default();
            let core_stats: Vec<&str> = core.split_whitespace().collect();
            cpu_core.name = core_stats[0].to_string();
            cpu_core.user = core_stats[1].parse::<u64>()?;
            cpu_core.nice = core_stats[2].parse::<u64>()?;
            cpu_core.system = core_stats[3].parse::<u64>()?;
            cpu_core.idle = core_stats[4].parse::<u64>()?;
            cpu_core.iowait = core_stats[5].parse::<u64>()?;
            cpu_core.irq = core_stats[6].parse::<u64>()?;
            cpu_core.softirq = core_stats[7].parse::<u64>()?;
            cores.push(cpu_core);
        }
        Ok(cores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let info = CPUInfo::new();
        println!("{info:?}");
    }
}
