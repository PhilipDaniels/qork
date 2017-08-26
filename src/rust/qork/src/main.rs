#![allow(dead_code)]
#![allow(warnings)]

extern crate chrono;
extern crate clap;
extern crate hostname;
extern crate lazy_init;
extern crate libc;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tempfile;
extern crate toml;
extern crate users;
extern crate xdg;

mod command;
mod command_line_arguments;
mod config_dir;
mod configuration;
mod context;
mod datetime;
mod execution_timer;
mod file;
mod mru_list;
mod program_info;
mod system_info;
mod runtime_data;

use std::io::{stdin};
use xdg::BaseDirectories;

use command::Command;
use config_dir::{ConfigDir, WellKnownDir};
use configuration::Configuration;
use context::Context;
use execution_timer::ExecutionTimer;
use program_info::ProgramInfo;
use runtime_data::RuntimeData;

// This produces various constants about the build environment which can be referred to using ::PKG_... syntax.
include!(concat!(env!("OUT_DIR"), "/built.rs"));

fn do_stuff() {
    let pi = ProgramInfo::new();
    let xdg = BaseDirectories::with_profile(::PKG_NAME, pi.parsed_args().xdg_profile()).unwrap();
    let cdir = ConfigDir::new(xdg.clone(), pi.parsed_args().load_config());
    info!("home() = {:?}", cdir.home());

    let mut p = cdir.create("foo.txt");//.unwrap();
    // info!("p = {:?}", p);
    // writeln!(p, "Hello file");
}

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
    let config = Configuration::load_user_configuration(&config_dir);
    let mut runtime_data = RuntimeData::load(&config, &xdg);

    let context = Context::new(pi, config_dir, config);
    info!("{:?}", context.system_info());

    run_event_loop(&context, &mut runtime_data);

    //runtime_data.save(context.configuration(), context.xdg());
}

fn configure_logging(xdg: &BaseDirectories) {
    let path = xdg.place_config_file("logging.toml");

    match path {
        Ok(p) => {
            if p.exists() {
                log4rs::init_file(&p, Default::default()).unwrap();
                info!("Logging initialized using file at {:?}", &p);
            }
        },
        Err(_) => {
            // Do nothing, not sure there is anything we can do.
        }
    }
}

fn log_build_info() {
    info!(r#"BuildInfo {{ PKG_VERSION: "{}", PROFILE: "{}", DEBUG: "{}", OPT_LEVEL: "{}", RUSTC: "{}", RUSTC_VERSION: "{}", FEATURES_STR: "{}", BUILD_TIME_UTC: "{}", GIT_VERSION: "{}", CFG_ARCH: "{}", CFG_ENDIAN: "{}", CFG_ENV: "{}", CFG_FAMILY: "{}", CFG_OS: "{}" }}"#,
        PKG_VERSION, PROFILE, DEBUG, OPT_LEVEL, RUSTC, RUSTC_VERSION, FEATURES_STR, BUILT_TIME_UTC, GIT_VERSION.unwrap(),
        CFG_TARGET_ARCH, CFG_ENDIAN, CFG_ENV, CFG_FAMILY, CFG_OS
        );
}

fn run_event_loop(context: &Context, runtime_data: &mut RuntimeData) {
    use std::io::BufRead;

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();

        let cmd = {
            if l == "q" {
                Command::Quit
            }
            else if l.starts_with("o ") {
                Command::OpenFile{ filename: l.chars().skip(2).collect() }
            }
            else {
                Command::NoOp
            }
        };

        let done = despatch_command(context, runtime_data, cmd);
        if done {
            break;
        }
    }
}

fn despatch_command(context: &Context, runtime_data: &mut RuntimeData, command: Command) -> bool {
    match command {
        Command::NoOp => { println!("Doing nothing"); }
        Command::Quit => { println!("Quitting"); return true; }
        Command::OpenFile{filename} => {
            println!("Opening file {}", filename);
            runtime_data.mru().insert(filename);
        }
    }

    false
}


// https://github.com/mkozachek/Rust-Events/blob/master/src/lib.rs