extern crate chrono;
extern crate clap;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate xdg;

mod command_line_arguments;
mod configuration;
mod context;
mod datetime;
mod execution_timer;
mod program_info;
mod system_info;

use xdg::BaseDirectories;

use context::Context;
use execution_timer::ExecutionTimer;

// This produces various constants about the build environment which can be referred to using ::PKG_... syntax.
include!(concat!(env!("OUT_DIR"), "/built.rs"));

fn main() {
    std::env::set_var("IN_QORK", "1");
    let context = Context::new();
    configure_logging(context.xdg());
    let _timer = ExecutionTimer::with_start_message("main.main");
    log_build_info();
    context.log_created_message();
    let config = configuration::Configuration::load_user_configuration(&context);
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
    info!(r#"BuildInfo {{ PKG_VERSION: "{}", PROFILE: "{}", DEBUG: "{}", OPT_LEVEL: "{}", RUSTC: "{}", RUSTC_VERSION: "{}", FEATURES_STR: "{}", BUILD_TIME_UTC: "{}", arch: "{}", endian: "{}", env: "{}", family: "{}", os: "{}" }}"#,
        PKG_VERSION, PROFILE, DEBUG, OPT_LEVEL, RUSTC, RUSTC_VERSION, FEATURES_STR, BUILT_TIME_UTC,
        CFG_TARGET_ARCH, CFG_ENDIAN, CFG_ENV, CFG_FAMILY, CFG_OS
        );
}
