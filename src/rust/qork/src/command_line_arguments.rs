use clap::{Arg, App};
use qork;

pub struct CommandLineArguments {
    xdg_profile: Option<String>
}

impl CommandLineArguments {
    pub fn new() -> CommandLineArguments {
        let matches = App::new("Qork")
                        .version(qork::VERSION)
                        .author("Philip Daniels philip.daniels1971@gmail.com")
                        .about("A text editor/IDE written in Rust and Python.")
                        .arg(Arg::with_name("xdg-profile")
                                .short("x")
                                .long("xdg-profile")
                                .value_name("DIRECTORY")
                                .help("Sets the XDG profile directory.")
                                .takes_value(true))
                        .get_matches();

        CommandLineArguments {
            xdg_profile: matches.value_of("xdg-profile").map(|s| s.to_string())
        }
    }

    pub fn xdg_profile(&self) -> &Option<String> {
        &self.xdg_profile
    }
}