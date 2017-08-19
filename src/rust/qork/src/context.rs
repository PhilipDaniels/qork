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
    configuration: Configuration,
    runtime_data: RuntimeData
}

impl Context {
    pub fn new(xdg: BaseDirectories, pi: ProgramInfo, config: Configuration, runtime_data: RuntimeData) -> Context {
        Context {
            xdg: xdg,
            system_info: SystemInfo::new(),
            program_info: pi,
            configuration: config,
            runtime_data: runtime_data
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

    pub fn runtime_data(&mut self) -> &mut RuntimeData {
        &mut self.runtime_data
    }
}
