use std;
use std::fmt;
use std::fs::Metadata;
use std::path::PathBuf;

use chrono::*;

use command_line_arguments::CommandLineArguments;
use datetime::*;
use qork;

// Information about the program (exe).
#[derive(Debug)]
pub struct ProgramInfo {
    version: &'static str,
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
            version: qork::VERSION,
            path: path,
            raw_args: std::env::args().collect(),
            parsed_args: CommandLineArguments::new(),
            meta_data: md
        }
    }

    pub fn version(&self) -> &'static str {
        &self.version
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

/*
impl<'a> fmt::Display for ProgramInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "version: {}, path: {}", &self.version, &self.path)
    }
}
*/