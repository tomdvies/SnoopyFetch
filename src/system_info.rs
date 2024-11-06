use std::env;
use sys_info;

#[derive(Debug)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub kernel_version: String,
    pub cpu_model: String,
    pub cpu_arch: String,
    pub memory_total: u64,
    pub memory_used: u64,
    pub uptime: u64,
    pub shell: String,
    pub brew_packages: usize,
}

pub fn collect_system_info() -> SystemInfo {
    SystemInfo {
        hostname: get_hostname(),
        os_name: get_os_name(),
        kernel_version: get_kernel_version(),
        cpu_model: get_cpu_info(),
        cpu_arch: get_cpu_arch(),
        memory_total: get_memory_total(),
        memory_used: get_memory_used(),
        uptime: get_uptime(),
        shell: get_shell(),
        brew_packages: get_brew_packages(),
    }
}

fn get_hostname() -> String {
    sys_info::hostname().unwrap_or_else(|_| String::from("Unknown"))
}

fn get_os_name() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(version) = sys_info::os_release() {
            return format!("macOS {}", version);
        }
        String::from("macOS")
    }
}

fn get_kernel_version() -> String {
    sys_info::os_release().unwrap_or_else(|_| String::from("Unknown"))
}

fn get_cpu_info() -> String {
    if let Ok(info) = sys_info::cpu_num() {
        format!("{} x CPU", info)
    } else {
        String::from("Unknown CPU")
    }
}

fn get_memory_total() -> u64 {
    sys_info::mem_info().map(|mem| mem.total).unwrap_or(0)
}

fn get_memory_used() -> u64 {
    sys_info::mem_info()
        .map(|mem| {
            // Total - (Free - avail) = Actually used memory
            mem.total.saturating_sub(mem.avail)
        })
        .unwrap_or(0)
}

fn get_cpu_arch() -> String {
    #[cfg(target_arch = "x86_64")]
    {
        String::from("x86_64")
    }
    #[cfg(target_arch = "aarch64")]
    {
        String::from("arm64")
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        String::from(env::consts::ARCH)
    }
}

fn get_uptime() -> u64 {
    if let Ok(boot_time) = sys_info::boottime() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(boot_time.tv_sec as u64)
    } else {
        0
    }
}

fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| String::from("Unknown"))
}

fn get_brew_packages() -> usize {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("brew")
            .arg("list")
            .output()
            .ok();
        
        if let Some(output) = output {
            if let Ok(list) = String::from_utf8(output.stdout) {
                return list.lines().count();
            }
        }
        0
    }
    #[cfg(not(target_os = "macos"))]
    {
        0
    }
}
