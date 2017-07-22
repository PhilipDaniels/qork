use std;
use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Utc, TimeZone, DateTime};
use command_line_arguments::CommandLineArguments;
use hostname;
use qork;
use slog::Logger;

// The complete execution context of Qork.
pub struct Context {
    pub logger: Logger,
    pub args: Vec<String>,
    pub exe_path: Option<std::path::PathBuf>,
    pub exe_meta_data: Option<std::fs::Metadata>,
    pub hostname: Option<String>,
    pub config_directory: PathBuf,
    pub command_line_arguments: CommandLineArguments
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

fn get_config_directory(logger: &Logger, args: &CommandLineArguments) -> PathBuf {
    const CONFIG_DIR_KEY : &'static str = "config_dir";
    const CONFIG_DIR : &'static str = ".qork.d";

    // Command line has highest priority.
    if let &Some(ref cd) = args.config_dir() {
        info!(logger, "Configuration Directory set from command line argument"; CONFIG_DIR_KEY => &cd);
        return PathBuf::from(&cd);
    }

    // Next is an environment variable.
    if let Ok(env_var) = std::env::var("QORK_CONFIG_DIR") {
        if !env_var.is_empty() {
            info!(logger, "Configuration Directory determined from QORK_CONFIG_DIR environment variable"; CONFIG_DIR_KEY => &env_var);
            return PathBuf::from(env_var);
        }
    }

    // If still no luck, try for '.qork.d' in the user's home directory - if we can
    // determine the user's home directory, that is.
    if let Some(mut home_dir) = env::home_dir() {
        home_dir.push(CONFIG_DIR);
        info!(logger, "Configuration Directory defaulted to '{}' in user's home directory", CONFIG_DIR; CONFIG_DIR_KEY => %&home_dir.display());
        return home_dir;
    }

    // Still stuck? Try .qork.d in the exe's directroy.
    if let Ok(mut exe_path) = std::env::current_exe() {
        exe_path.pop();
        exe_path.push(CONFIG_DIR);
        info!(logger, "Configuration Directory defaulted to '{}' in directory of the exe", CONFIG_DIR; CONFIG_DIR_KEY => %&exe_path.display());
        return exe_path;
    }

    // Last, just default to .qork.d in the current working directory.
    warn!(logger, "Unable to determine Configuration Directory from command line parameter, home dir, or exe dir: defaulting to '{}' in the current working directory", CONFIG_DIR;
        CONFIG_DIR_KEY => CONFIG_DIR);
    PathBuf::from(CONFIG_DIR)
}

impl Context {
    pub fn new(logger: Logger, args: CommandLineArguments) -> Context {
        let exe = std::env::current_exe().ok();
        let md = exe.as_ref().and_then(|e| e.metadata().ok());
        let cd = get_config_directory(&logger, &args);

        Context {
            logger: logger,
            args: std::env::args().collect(),
            exe_path: exe,
            exe_meta_data: md,
            hostname: hostname::get_hostname(),
            config_directory: cd,
            command_line_arguments: args
        }
    }

    pub fn version(&self) -> &'static str {
        qork::VERSION
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
               "config_directory" => &self.config_directory.to_str(),
               "version" => self.version(),
               "hostname" => &self.hostname,
               "exe_modified" => mdate,
               "exe_bytes" => bytes,
               "exe_path" => p
         );
    }
}