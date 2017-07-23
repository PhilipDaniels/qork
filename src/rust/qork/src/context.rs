use std;
use std::env;
use std::fs;
use std::path::PathBuf;
use command_line_arguments::CommandLineArguments;
use datetime::system_time_to_date_time;
use hostname;
use qork;
use slog::Logger;
use xdg::BaseDirectories;

// The complete execution context of Qork.
pub struct Context {
    logger: Logger,
    // The raw command line arguments.
    args: Vec<String>,
    // Parsed form of the arguments.
    command_line_arguments: CommandLineArguments,
    exe_path: Option<PathBuf>,
    exe_meta_data: Option<fs::Metadata>,
    hostname: Option<String>,
    // xdg base dir object, typically '~/.config/qork', with a default profile of
    // 'default', which means the effective directory is '~/.config/qork/default'
    xdg: BaseDirectories,
    // TODO: system_type, user_name
}

impl Context {
    pub fn new(logger: Logger, args: CommandLineArguments) -> Context {
        let exe = std::env::current_exe().ok();
        let md = exe.as_ref().and_then(|e| e.metadata().ok());

        let profile = {
            match args.xdg_profile()
            {
                &Some(ref p) => { p.clone().to_string() },
                &None => "default".to_string()
            }
        };

        let bd = BaseDirectories::with_profile(qork::APP_NAME, profile).unwrap();

        Context {
            logger: logger,
            args: std::env::args().collect(),
            exe_path: exe,
            exe_meta_data: md,
            hostname: hostname::get_hostname(),
            command_line_arguments: args,
            xdg: bd
        }
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn command_line_arguments(&self) -> &CommandLineArguments {
        &self.command_line_arguments
    }

    pub fn exe_path(&self) -> &Option<PathBuf> {
        &self.exe_path
    }

    pub fn exe_meta_data(&self) -> &Option<fs::Metadata> {
        &self.exe_meta_data
    }

    pub fn hostname(&self) -> &Option<String> {
        &self.hostname
    }

    pub fn xdg(&self) -> &BaseDirectories {
        &self.xdg
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
               "config_directory" => %&self.xdg.get_config_home().display(),
               "version" => self.version(),
               "hostname" => &self.hostname,
               "exe_modified" => mdate,
               "exe_bytes" => bytes,
               "exe_path" => p
         );
    }
}