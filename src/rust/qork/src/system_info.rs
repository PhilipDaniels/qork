use hostname;
use target_info;

// Information about the system/machine we are running on.
// See https://doc.rust-lang.org/beta/reference/attributes.html#conditional-compilation
// and the target_info crate. Most of these fields are compile-time constants.
#[derive(Debug)]
pub struct SystemInfo {
    hostname: String,
    // e.g. x86, x86_64, mips...
    pub arch: &'static str,
    // little or big.
    pub endian: &'static str,
    // e.g. gnu, msvc, musl.
    pub env: &'static str,
    // e.g. unix, windows.
    pub family: &'static str,
    // e.g. linux, windows, macos, ios, android.
    pub os: &'static str
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            hostname: hostname::get_hostname().unwrap_or(String::new()),
            arch: target_info::Target::arch(),
            endian: target_info::Target::endian(),
            env: target_info::Target::env(),
            family: target_info::Target::family(),
            os: target_info::Target::os()
        }
    }

    pub fn hostname(&self) -> &String {
        &self.hostname
    }
}
