use std::cell::{RefCell, RefMut, Ref};
use configuration::Configuration;
use buffer::BufferCollection;
use fs::ConfigDir;
use persistent_state::PersistentState;
use program_info::ProgramInfo;
use system_info::SystemInfo;

/// The complete execution context of Qork.
pub struct Context {
    system_info: SystemInfo,
    program_info: ProgramInfo,
    config_dir: ConfigDir,
    configuration: Configuration,
    state: RefCell<PersistentState>,
    buffers: RefCell<BufferCollection>
}

impl Context {
    pub fn new(pi: ProgramInfo, config_dir: ConfigDir, config: Configuration, state: PersistentState) -> Context {
        Context {
            system_info: SystemInfo::new(),
            program_info: pi,
            config_dir: config_dir,
            configuration: config,
            state: RefCell::new(state),
            buffers: RefCell::new(BufferCollection::new())
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

    pub fn state(&self) -> RefMut<PersistentState> {
        self.state.borrow_mut()
    }

    pub fn buffers(&self) -> Ref<BufferCollection> {
        self.buffers.borrow()
    }
}

