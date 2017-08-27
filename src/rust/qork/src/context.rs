use configuration::Configuration;
use fs::{ConfigDir};
use program_info::ProgramInfo;
use runtime_data::RuntimeData;
use system_info::SystemInfo;

/// The complete execution context of Qork.
pub struct Context {
    system_info: SystemInfo,
    program_info: ProgramInfo,
    config_dir: ConfigDir,
    configuration: Configuration
}

impl Context {
    pub fn new(pi: ProgramInfo, config_dir: ConfigDir, config: Configuration) -> Context {
        Context {
            system_info: SystemInfo::new(),
            program_info: pi,
            config_dir: config_dir,
            configuration: config,
        }
    }

    pub fn system_info(&self) -> &SystemInfo {
        &self.system_info
    }

    pub fn program_info(&self) -> &ProgramInfo {
        &self.program_info
    }

    pub fn config_dir(&self) -> &ConfigDir {
        &self.config_dir
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
