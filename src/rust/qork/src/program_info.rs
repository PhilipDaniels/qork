use std;
use std::fmt;
use std::fs::Metadata;
use std::path::PathBuf;

use chrono::prelude::*;
use libc;
use libc::{pid_t, uid_t, gid_t};

use command_line_arguments::CommandLineArguments;
use datetime::*;

// Information about the program we are running (qork.exe), the invocation.
// This information is derived at runtime.

pub struct ProgramInfo {
    path: Option<PathBuf>,
    meta_data: Option<Metadata>,
    raw_args: Vec<String>,
    parsed_args: CommandLineArguments,
    pub pid: pid_t,
    pub parent_pid: pid_t,
    pub uid: uid_t,
    pub effective_uid: uid_t,
    pub gid: gid_t,
    pub effective_gid: gid_t
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
            uid: unsafe { libc::getuid() },
            effective_uid: unsafe { libc::getuid() },
            gid: unsafe { libc::getgid() },
            effective_gid: unsafe { libc::getegid() },
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

    pub fn modified_date(&self) -> Option<DateTime<Utc>> {
         self.meta_data.as_ref().map(|m| m.modified().ok()).map(|m| system_time_to_date_time(m.unwrap()))
    }
}

impl fmt::Debug for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match &self.path {
            &Some(ref pb) => pb.to_str().unwrap_or("unknown"),
            &None => "unknown"
        };

        let mdate = match &self.modified_date() {
            &Some(t) => t.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string(),
            &None => String::from("unknown")
        };

        write!(f, r#"ProgramInfo {{ path: "{}", size: {}, modified_date: "{}", pid: {}, parent_pid: {}, uid: {}, effective_uid: {}, gid: {}, effective_gid: {} }}"#,
            p,
            self.size().unwrap_or(0),
            mdate,
            self.pid, self.parent_pid,
            self.uid, self.effective_uid,
            self.gid, self.effective_gid
        )
    }
}

impl fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
