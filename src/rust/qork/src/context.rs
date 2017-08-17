use xdg::BaseDirectories;

use configuration::Configuration;
use program_info::ProgramInfo;
use system_info::SystemInfo;

/// The complete execution context of Qork.
pub struct Context {
    system_info: SystemInfo,
    program_info: ProgramInfo,
    // xdg base directory object, typically '~/.config/qork', with a default profile of
    // 'default', which means the effective directory is '~/.config/qork/default'.
    xdg: BaseDirectories,
    config: Configuration
}

impl Context {
    pub fn new() -> Context {
        let pi = ProgramInfo::new();

        let bd = BaseDirectories::with_profile(::PKG_NAME, pi.parsed_args().xdg_profile()).unwrap();

        Context {
            xdg: bd,
            system_info: SystemInfo::new(),
            program_info: pi,
            config: Configuration::default()
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
        info!("{:?}", self.system_info);
        info!("{:?}", self.program_info.parsed_args());
        info!("{:?}", self.program_info);
    }
}
