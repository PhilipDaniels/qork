use std;
use std::env;
use std::fs;
use std::path::PathBuf;
use command_line_arguments::CommandLineArguments;
use datetime::system_time_to_date_time;
use hostname;
use qork;
use slog::Logger;

// The complete execution context of Qork.
pub struct Context {
    logger: Logger,
    args: Vec<String>,
    exe_path: Option<PathBuf>,
    exe_meta_data: Option<fs::Metadata>,
    hostname: Option<String>,
    config_directory: PathBuf,
    command_line_arguments: CommandLineArguments
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

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
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

    pub fn config_directory(&self) -> &PathBuf {
        &self.config_directory
    }

    pub fn command_line_arguments(&self) -> &CommandLineArguments {
        &self.command_line_arguments
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