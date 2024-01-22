use std::fs;

pub struct CpuInfo {
    pub name: String,
    pub num_cores: i32,
}
impl CpuInfo {
    fn new() -> Self {
        Self {
            name: String::new(),
            num_cores: 0,
        }
    }
}

pub fn get_cpu_info() -> Option<CpuInfo> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").ok()?;

    let mut cpu_info = CpuInfo::new();

    for line in cpuinfo.lines() {
        if let Some((key, value)) = parse_cpu_info_line(line) {
            match key {
                "model name" => cpu_info.name = value.to_string(),
                "cpu cores" => cpu_info.num_cores = value.parse().unwrap_or(0),
                _ => {}
            }
        }
    }

    if cpu_info.name.is_empty() || cpu_info.num_cores == 0 {
        None
    } else {
        Some(cpu_info)
    }
}

fn parse_cpu_info_line(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();

    if parts.len() == 2 {
        Some((parts[0], parts[1]))
    } else {
        None
    }
}
