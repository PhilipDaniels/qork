use std;
use std::fmt;
use std::fs::Metadata;
use std::path::PathBuf;

use chrono::prelude::*;

use command_line_arguments::CommandLineArguments;
use datetime::*;

// Information about the program (exe).
pub struct ProgramInfo {
    pub version: &'static str,
    path: Option<PathBuf>,
    raw_args: Vec<String>,
    parsed_args: CommandLineArguments,
    meta_data: Option<Metadata>
}

impl ProgramInfo {
    pub fn new() -> ProgramInfo {
        let path = std::env::current_exe().ok();
        let md = path.as_ref().and_then(|e| e.metadata().ok());

        ProgramInfo {
            version: crate_version!(),
            path: path,
            raw_args: std::env::args().collect(),
            parsed_args: CommandLineArguments::new(),
            meta_data: md
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

        write!(f, "{{ version={}, path={}, size={}, modified_date={} }}",
            self.version,
            p,
            self.size().unwrap_or(0),
            mdate
        )
    }
}

impl fmt::Display for ProgramInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
