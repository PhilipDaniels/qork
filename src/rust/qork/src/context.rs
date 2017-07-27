use xdg::BaseDirectories;

use command_line_arguments::CommandLineArguments;
use program_info::ProgramInfo;
use qork;
use system_info::SystemInfo;

// The complete execution context of Qork.
#[derive(Debug)]
pub struct Context {
    // xdg base dir object, typically '~/.config/qork', with a default profile of
    // 'default', which means the effective directory is '~/.config/qork/default'
    xdg: BaseDirectories,
    system_info: SystemInfo,
    program_info: ProgramInfo
    // TODO: user_name
}

impl Context {
    pub fn new(args: CommandLineArguments) -> Context {
        let profile = {
            match args.xdg_profile()
            {
                &Some(ref p) => { p.clone().to_string() },
                &None => "default".to_string()
            }
        };

        let bd = BaseDirectories::with_profile(qork::APP_NAME, profile).unwrap();

        Context {
            xdg: bd,
            system_info: SystemInfo::new(),
            program_info: ProgramInfo::new()
        }
    }

    pub fn system_info(&self) -> &SystemInfo {
        &self.system_info
    }

    pub fn program_info(&self) -> &ProgramInfo {
        &self.program_info
    }

    pub fn xdg(&self) -> &BaseDirectories {
        &self.xdg
    }

    pub fn log_created_message(&self) -> () {
        let mdate = &self.program_info.modified_date().map(|m| m.format("%Y-%m-%d %H:%M:%S%.3f UTC").to_string());
        let path = &self.program_info.path().as_ref().and_then(|p| p.to_str()).map(String::from);

        // info!(self.logger, "Created Context";
        //     "system_info.hostname" => self.system_info.hostname(),
        //     "system_info.arch" => self.system_info.arch(),
        //     "system_info.endian" => self.system_info.endian(),
        //     "system_info.env" => self.system_info.env(),
        //     "system_info.family" => self.system_info.family(),
        //     "system_info.os" => self.system_info.os(),
        //     "config_directory" => %&self.xdg.get_config_home().display(),
        //     "program_info.version" => &self.program_info.version(),
        //     "program_info.modified_date" => mdate,
        //     "program_info.size" => &self.program_info.size(),
        //     "program_info.path" => path
        //  );
    }
}