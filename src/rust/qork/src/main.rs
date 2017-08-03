extern crate chrono;
#[macro_use]
extern crate clap;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate target_info;
extern crate xdg;

mod command_line_arguments;
mod context;
mod datetime;
mod execution_timer;
mod program_info;
mod system_info;

// The file has been placed there by the build.rs script and the 'built' crate.
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use xdg::BaseDirectories;

use context::Context;
use execution_timer::ExecutionTimer;

fn main() {
    std::env::set_var("IN_QORK", "1");
    let context = Context::new();
    configure_logging(context.xdg());
    let _timer = ExecutionTimer::with_start_message("main.main");
    context.log_created_message();
    load_user_configuration_if_valid(&context);

    // Keep, works.
    info!("PKG_DESCRIPTION = {}", built_info::PKG_DESCRIPTION);
    info!("PKG_NAME = {}", built_info::PKG_NAME);
    info!("PKG_VERSION = {}", built_info::PKG_VERSION);
    info!("PKG_AUTHORS = {}", built_info::PKG_AUTHORS);
    info!("PROFILE = {}", built_info::PROFILE);
    info!("DEBUG = {}", built_info::DEBUG);
    info!("OPT_LEVEL = {}", built_info::OPT_LEVEL);
    info!("FEATURES_STR = {}", built_info::FEATURES_STR);
    info!("BUILT_TIME_UTC = {}", built_info::BUILT_TIME_UTC);

    info!("NEED FIXING >>>>>>>>>>>>>>>>>>");
    info!("RUSTC_VERSION = {:?}", built_info::RUSTC_VERSION);
    info!("GIT_VERSION = {:?}", built_info::GIT_VERSION);
    info!("CI_PLATFORM = {:?}", built_info::CI_PLATFORM);

    // Do not use, duplicate of target_info.
    info!("DO NOT USE >>>>>>>>>>>>>>>>>>");
    info!("TARGET = {}", built_info::TARGET);
    info!("HOST = {}", built_info::HOST);
}

fn configure_logging(xdg: &BaseDirectories) {
    let path = xdg.place_config_file("logging.toml");

    match path {
        Ok(p) => {
            if p.exists() {
                log4rs::init_file(&p, Default::default()).unwrap();
                info!("Logging initialized using file at {:?}", &p);
            }
            else {
            }
        },
        Err(_) => {
            // Do nothing, not sure there is anything we can do.
        }
    }
}

fn load_user_configuration_if_valid(context: &Context) {
    if !context.program_info().parsed_args().load_config() {
        info!("Loading of user configuration is disabled.");
        return
    }

    let dir = context.xdg().get_config_home();
    if !dir.exists() {
        warn!("The config_directory does not exist, no config will be loaded, config_directory={:?}", dir);
        return
    }

    if !dir.is_dir() {
        warn!("The config_directory is a file, not a directory, no config will be loaded, config_directory={:?}", dir);
        return
    }

    let _timer = ExecutionTimer::with_start_message("main.load_user_configuration_if_valid");
    info!("Loading user configuration from {:?}", dir);
}
