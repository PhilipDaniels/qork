use clap::{Arg, App};
use qork;

pub struct CommandLineArguments {
    config_dir: Option<String>
}

impl CommandLineArguments {
    pub fn new() -> CommandLineArguments {
        let matches = App::new("Qork")
                        .version(qork::VERSION)
                        .author("Philip Daniels philip.daniels1971@gmail.com")
                        .about("A text editor/IDE written in Rust and Python.")
                        .arg(Arg::with_name("config-dir")
                                .short("c")
                                .long("config-dir")
                                .value_name("DIRECTORY")
                                .help("Sets the configuration directory. Use blank to avoid loading any config.")
                                .takes_value(true))
                        .get_matches();

        CommandLineArguments {
            config_dir: matches.value_of("config-dir").map(|s| s.to_string())
        }
    }

    pub fn config_dir(&self) -> &Option<String> {
        &self.config_dir
    }
}