use hostname;
use target_info;

// Information about the system/machine we are running on.
// See https://doc.rust-lang.org/beta/reference/attributes.html#conditional-compilation
// and the target_info crate. Most of these fields are compile-time constants.
#[derive(Debug)]
pub struct SystemInfo {
    hostname: String,
    // e.g. x86, x86_64, mips...
    arch: &'static str,
    // little or big.
    endian: &'static str,
    // e.g. gnu, msvc, musl.
    env: &'static str,
    // e.g. unix, windows.
    family: &'static str,
    // e.g. linux, windows, macos, ios, android.
    os: &'static str
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

    // We are returning a shared (immutable) reference.
    // There is no way for the caller to modify this string.
    pub fn hostname(&self) -> &String {
        &self.hostname
    }

    // You only get to specify the mutability in the return type of your function if you're returning
    // a reference to (aka borrow of) an object, rather than the object itself.
    pub fn mut_hostname(&mut self) -> &mut String {
        &mut self.hostname
    }

    pub fn arch(&self) -> &'static str {
        self.arch
    }

    pub fn endian(&self) -> &'static str {
        self.endian
    }

    pub fn env(&self) -> &'static str {
        self.env
    }

    pub fn family(&self) -> &'static str {
        self.family
    }

    pub fn os(&self) -> &'static str {
        self.os
    }
}