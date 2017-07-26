use slog::Logger;
use xdg::BaseDirectories;

use command_line_arguments::CommandLineArguments;
use program_info;
use qork;
use system_info;

// The complete execution context of Qork.
#[derive(Debug)]
pub struct Context {
    logger: Logger,
    // xdg base dir object, typically '~/.config/qork', with a default profile of
    // 'default', which means the effective directory is '~/.config/qork/default'
    xdg: BaseDirectories,
    pub system_info: system_info::SystemInfo,
    pub program_info: program_info::ProgramInfo
    // TODO: user_name
}

// impl slog::Value for std::path::PathBuf { }

impl Context {
    pub fn new(logger: Logger, args: CommandLineArguments) -> Context {
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
            xdg: bd,
            system_info: system_info::SystemInfo::new(),
            program_info: program_info::ProgramInfo::new()
        }
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn xdg(&self) -> &BaseDirectories {
        &self.xdg
    }

    pub fn log_created_message(&self) -> () {
        let mdate = &self.program_info.modified_date().map(|m| m.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string());
        let path = &self.program_info.path.as_ref().and_then(|p| p.to_str()).map(String::from);

        info!(self.logger, "Created Context";
            "system_info.hostname" => &self.system_info.hostname,
            "system_info.arch" => &self.system_info.arch,
            "system_info.endian" => &self.system_info.endian,
            "system_info.env" => &self.system_info.env,
            "system_info.family" => &self.system_info.family,
            "system_info.os" => &self.system_info.os,
            "config_directory" => %&self.xdg.get_config_home().display(),
            "program_info.version" => &self.program_info.version,
            "program_info.modified_date" => mdate,
            "program_info.size" => &self.program_info.size(),
            "program_info.path" => path
         );
    }
}