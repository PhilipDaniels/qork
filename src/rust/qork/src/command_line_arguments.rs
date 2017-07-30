use clap::{Arg, App};
use qork;

#[derive(Debug)]
pub struct CommandLineArguments {
    load_config: bool,
    xdg_profile: Option<String>
}

impl CommandLineArguments {
    pub fn new() -> CommandLineArguments {
        let matches = App::new("Qork")
                        .version(qork::VERSION)
                        .author("Philip Daniels philip.daniels1971@gmail.com")
                        .about("A text editor/IDE written in Rust and Python.")
                        .arg(Arg::with_name("xdg-profile")
                                .short("p")
                                .long("xdg-profile")
                                .value_name("DIRECTORY")
                                .help("Sets the XDG profile directory.")
                                .takes_value(true)
                        )
                        .arg(Arg::with_name("no-config")
                                .short("q")
                                .long("no-config")
                                .help("Prevent the loading of the configuration directory")
                        )
                        .get_matches();

        CommandLineArguments {
            // Flip this so that we express what we want to do as a positive boolean (double
            // negatives are hard to reason about). The code will read 'if args.load_config()...'
            load_config: !matches.is_present("no-config"),
            xdg_profile: matches.value_of("xdg-profile").map(|s| s.to_string())
        }
    }

    pub fn load_config(&self) -> bool {
        self.load_config
    }

    pub fn xdg_profile(&self) -> &Option<String> {
        &self.xdg_profile
    }
}