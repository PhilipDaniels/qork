use hostname;

/// Information about the system/machine we are running on.
/// This information is derived at runtime.
#[derive(Debug, Default)]
pub struct SystemInfo {
    hostname: String
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            hostname: hostname::get_hostname().unwrap_or_default()
        }
    }

    pub fn hostname(&self) -> &String {
        &self.hostname
    }
}
