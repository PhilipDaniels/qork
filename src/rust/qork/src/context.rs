use std;

// The complete execution context of Qork.
pub struct Context {
    pub args: Vec<String>,
    pub exe_path: std::path::PathBuf,
    pub exe_meta_data: std::fs::Metadata
}

impl Context {
    pub fn new() -> Context {
        let p = std::env::current_exe();

        let exe = match p {
            Ok(exe_path) => exe_path,
            Err(e) => std::path::PathBuf::new()
        };

        let md = match exe.metadata() {
            Ok(md) => md,
            Err(e) => panic!("Cannot determine exe meta data.")
        };


        Context {
            args: std::env::args().collect(),
            exe_path: exe,
            exe_meta_data: md
        }
    }
}