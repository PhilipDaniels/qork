use hostname;

// Information about the system/machine we are running on.
// This information is derived at runtime.

// TODO: Replace this with the sysinfo crate.

#[derive(Debug)]
pub struct SystemInfo {
    hostname: String,
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
