use hostname;
use libc::pid_t;
use sysinfo;
use sysinfo::{get_current_pid, SystemExt};

// Information about the system/machine we are running on.
// This information is derived at runtime.

#[derive(Debug, Default)]
pub struct SystemInfo {
    hostname: String
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            hostname: hostname::get_hostname().unwrap_or(String::new()),
        }
    }

    pub fn hostname(&self) -> &String {
        &self.hostname
    }
}
