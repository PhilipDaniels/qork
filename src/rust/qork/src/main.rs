#![allow(dead_code)]
//#![allow(warnings)]

extern crate chrono;
extern crate clap;
extern crate hostname;
extern crate lazy_init;
extern crate libc;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate shellexpand;
extern crate tempfile;
extern crate toml;
extern crate users;
extern crate xdg;
extern crate xi_rope;

mod buffer;
mod commands;
mod command_line_arguments;
mod configuration;
mod fs;
mod context;
mod datetime;
mod execution_timer;
mod mru_list;
mod program_info;
mod system_info;
mod persistent_state;
mod utils;

use std::io::{stdin};
use xdg::BaseDirectories;

use commands::{handle_command, parse_command};
use configuration::Configuration;
use fs::{ConfigDir, DataDir};
use context::Context;
use execution_timer::ExecutionTimer;
use program_info::ProgramInfo;
use persistent_state::PersistentState;

// This produces various constants about the build environment which can be referred to using ::PKG_... syntax.
include!(concat!(env!("OUT_DIR"), "/built.rs"));

fn main() {
    std::env::set_var("IN_QORK", "1");

    // Configure logging as early as possible (because, obviously, we want to log in the rest of the initialization process).
    let pi = ProgramInfo::new();
    let xdg = BaseDirectories::with_profile(::PKG_NAME, pi.parsed_args().xdg_profile()).unwrap();
    configure_logging(&xdg);

    let _timer = ExecutionTimer::with_start_message("main.main");
    log_build_info();
    info!("{:?}", pi.parsed_args());
    info!("{:?}", pi);

    let config_dir = ConfigDir::new(xdg.clone(), pi.parsed_args().load_config());
    let data_dir = DataDir::new(xdg.clone(), pi.parsed_args().load_config());
    let config = Configuration::load_user_configuration(&config_dir);
    let persistent_state = PersistentState::load(&config, &data_dir);
    let context = Context::new(pi, config_dir, config, persistent_state);
    info!("{:?}", context.system_info());

    run_event_loop(&context);

    context.state().save(&data_dir);
}

fn configure_logging(xdg: &BaseDirectories) {
    if let Ok(path) = xdg.place_config_file("logging.toml") {
        if path.exists() {
            log4rs::init_file(&path, Default::default()).unwrap();
            info!("Logging initialized using file at {:?}", &path);
        }
    }
}

fn log_build_info() {
    info!(r#"BuildInfo {{ PKG_VERSION: "{}", PROFILE: "{}", DEBUG: "{}", OPT_LEVEL: "{}", RUSTC: "{}", RUSTC_VERSION: "{}", FEATURES_STR: "{}", BUILD_TIME_UTC: "{}", GIT_VERSION: "{}", CFG_ARCH: "{}", CFG_ENDIAN: "{}", CFG_ENV: "{}", CFG_FAMILY: "{}", CFG_OS: "{}" }}"#,
        PKG_VERSION, PROFILE, DEBUG, OPT_LEVEL, RUSTC, RUSTC_VERSION, FEATURES_STR, BUILT_TIME_UTC, GIT_VERSION.unwrap(),
        CFG_TARGET_ARCH, CFG_ENDIAN, CFG_ENV, CFG_FAMILY, CFG_OS
        );
}

fn run_event_loop(context: &Context) {
    use std::io::BufRead;

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let cmd = parse_command(&l);
        if handle_command(context, cmd) {
            break;
        }
    }
}