use std::fmt;
use clap::{Arg, App};

// This struct represents the parsed command line arguments, not the raw ones.
// For example, if no value is supplied for an argument then the default is applied
// and set for the corresponding field in this struct.
pub struct CommandLineArguments {
    load_config: bool,
    xdg_profile: String
}

impl CommandLineArguments {
    pub fn new() -> CommandLineArguments {
        let matches = App::new(::PKG_NAME)
                        .version(::PKG_VERSION)
                        .author(::PKG_AUTHORS)
                        .about(::PKG_DESCRIPTION)
                        .arg(Arg::with_name("xdg-profile")
                                .short("p")
                                .long("xdg-profile")
                                .value_name("DIRECTORY")
                                .help("Sets the XDG profile directory.")
                                .default_value("default")
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
            xdg_profile: String::from(matches.value_of("xdg-profile").unwrap())
        }
    }

    pub fn load_config(&self) -> bool {
        self.load_config
    }

    pub fn xdg_profile(&self) -> &String {
        &self.xdg_profile
    }
}

impl fmt::Debug for CommandLineArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"CommandLineArguments {{ load_config: {}, xdg_profile: "{}" }}"#,
            self.load_config, self.xdg_profile
        )
    }
}

impl fmt::Display for CommandLineArguments {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}