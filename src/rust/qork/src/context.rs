use std;
use slog::Logger;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Utc, TimeZone, DateTime};
use hostname;

// The complete execution context of Qork.
pub struct Context {
    pub logger: Logger,
    pub args: Vec<String>,
    pub exe_path: Option<std::path::PathBuf>,
    pub exe_meta_data: Option<std::fs::Metadata>,
    pub hostname: Option<String>
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Utc.timestamp(sec, nsec)
}

impl Context {
    pub fn new(logger: Logger) -> Context {
        let exe = std::env::current_exe().ok();
        let md = exe.as_ref().and_then(|e| e.metadata().ok());

        Context {
            logger: logger,
            args: std::env::args().collect(),
            exe_path: exe,
            exe_meta_data: md,
            hostname: hostname::get_hostname(),
        }
    }

    pub fn version(&self) -> &'static str {
        "0.1.0"
    }

    pub fn log_created_message(&self) -> () {
        let p = self.exe_path.as_ref()
            .and_then(|p| p.to_str())
            .unwrap_or("unknown");

        let bytes = self.exe_meta_data.as_ref()
            .map(|m| m.len().to_string())
            .unwrap_or("unknown".to_string());

        let mdate = self.exe_meta_data.as_ref()
            .and_then(|md| md.modified().ok())
            .map(|md| system_time_to_date_time(md))
            .map(|md| md.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string())
            .unwrap_or("unknown".to_string());

        info!(self.logger, "Created Context";
               "version" => self.version(),
               "hostname" => &self.hostname,
               "exe_modified" => mdate,
               "exe_bytes" => bytes,
               "exe_path" => p
         );
    }
}