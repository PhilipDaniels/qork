use std;
use std::fmt;
use std::fs::Metadata;
use std::path::PathBuf;
use std::time::SystemTime;

use libc;
use libc::{pid_t};
use users;
use users::{uid_t, gid_t};

use command_line_arguments::CommandLineArguments;
use datetime::{format_system_time_as_utc};

/// Information about the program we are running (qork.exe), the invocation.
/// This information is initialized at program startup. It is never refreshed,
/// and so may become stale.
/// TODO: Should we call the function every time instead of caching these values?
pub struct ProgramInfo {
    path: Option<PathBuf>,
    meta_data: Option<Metadata>,
    raw_args: Vec<String>,
    parsed_args: CommandLineArguments,
    pub pid: pid_t,
    pub parent_pid: pid_t,
    pub uid: uid_t,
    pub uid_name: Option<String>,
    pub effective_uid: uid_t,
    pub effective_uid_name: Option<String>,
    pub gid: gid_t,
    pub gid_name: Option<String>,
    pub effective_gid: gid_t,
    pub effective_gid_name: Option<String>
}

impl ProgramInfo {
    pub fn new() -> ProgramInfo {
        let path = std::env::current_exe().ok();
        let md = path.as_ref().and_then(|e| e.metadata().ok());

        ProgramInfo {
            path: path,
            meta_data: md,
            raw_args: std::env::args().collect(),
            parsed_args: CommandLineArguments::new(),
            pid: unsafe { libc::getpid() },
            parent_pid: unsafe { libc::getppid() },
            uid: users::get_current_uid(),
            uid_name: users::get_current_username(),
            effective_uid: users::get_effective_uid(),
            effective_uid_name: users::get_effective_username(),
            gid: users::get_current_gid(),
            gid_name: users::get_current_groupname(),
            effective_gid: users::get_effective_gid(),
            effective_gid_name: users::get_effective_groupname()
        }
    }

    pub fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub fn raw_args(&self) -> &Vec<String> {
        &self.raw_args
    }

    pub fn parsed_args(&self) -> &CommandLineArguments {
        &self.parsed_args
    }

    pub fn meta_data(&self) -> &Option<Metadata> {
        &self.meta_data
    }

    pub fn size(&self) -> Option<u64> {
        self.meta_data.as_ref().map(|m| m.len())
    }

    pub fn modified_date(&self) -> Option<SystemTime> {
        self.meta_data().as_ref().and_then(|md| md.modified().ok())
    }
}

impl fmt::Debug for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self.path {
            Some(ref pb) => pb.to_str().unwrap_or("unknown"),
            None => "unknown"
        };

        let mdate = self.modified_date().map_or(String::from("unknown"), |d| format_system_time_as_utc(&d));

        write!(f, "ProgramInfo {{ path: \"{}\", size: {}, modified_date: \"{}\", pid: {}, parent_pid: {}, uid: {}, uid_name: {:?}, \
        effective_uid: {}, effective_uid_name: {:?} gid: {}, gid_name: {:?}, effective_gid: {}, effective_gid_name: {:?} }}",
            p,
            self.size().unwrap_or(0),
            mdate,
            self.pid, self.parent_pid,
            self.uid, self.uid_name, self.effective_uid, self.effective_uid_name,
            self.gid, self.gid_name, self.effective_gid, self.effective_gid_name
        )
    }
}

impl fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
