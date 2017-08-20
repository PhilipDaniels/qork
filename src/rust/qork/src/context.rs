use xdg::BaseDirectories;

use configuration::Configuration;
use program_info::ProgramInfo;
use runtime_data::RuntimeData;
use system_info::SystemInfo;

/// The complete execution context of Qork.
pub struct Context {
    system_info: SystemInfo,
    program_info: ProgramInfo,
    // xdg base directory object, typically '~/.config/qork', with a default profile of
    // 'default', which means the effective directory is '~/.config/qork/default'.
    xdg: BaseDirectories,
    configuration: Configuration
}

impl Context {
    pub fn new(xdg: BaseDirectories, pi: ProgramInfo, config: Configuration) -> Context {
        Context {
            xdg: xdg,
            system_info: SystemInfo::new(),
            program_info: pi,
            configuration: config,
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

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
