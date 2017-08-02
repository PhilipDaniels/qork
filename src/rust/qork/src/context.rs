use xdg::BaseDirectories;

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
    pub fn new() -> Context {
        let pi = ProgramInfo::new();

        let bd = BaseDirectories::with_profile(qork::APP_NAME, pi.parsed_args().xdg_profile()).unwrap();

        Context {
            xdg: bd,
            system_info: SystemInfo::new(),
            program_info: pi
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
        info!("Qork Context Created. program_info = {}", self.program_info);
        info!("Parsed command line args = {}", self.program_info.parsed_args());

        // info!(self.logger, "Created Context";
        //     "system_info.hostname" => self.system_info.hostname(),
        //     "system_info.arch" => self.system_info.arch(),
        //     "system_info.endian" => self.system_info.endian(),
        //     "system_info.env" => self.system_info.env(),
        //     "system_info.family" => self.system_info.family(),
        //     "system_info.os" => self.system_info.os(),
        //     "config_directory" => %&self.xdg.get_config_home().display(),
        //  );
    }
}