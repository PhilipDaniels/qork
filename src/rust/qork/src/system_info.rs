use hostname;
use libc::pid_t;
use sysinfo;
use sysinfo::{get_current_pid, SystemExt};

// Information about the system/machine we are running on.
// This information is derived at runtime.

// TODO: Replace this with the sysinfo crate.

#[derive(Debug, Default)]
pub struct SystemInfo {
    pid: pid_t,
    parent_pid: Option<pid_t>,
    hostname: String,
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        let mut system = sysinfo::System::new();
        let pid = sysinfo::get_current_pid();
        let process = system.get_process(pid).unwrap();

        SystemInfo {
            pid: pid,
            parent_pid: process.parent,
            hostname: hostname::get_hostname().unwrap_or(String::new()),
        }
    }

    pub fn hostname(&self) -> &String {
        &self.hostname
    }

    pub fn pid(&self) -> pid_t {
        self.pid
    }
}
