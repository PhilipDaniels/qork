extern crate chrono;
#[macro_use]
extern crate clap;
extern crate hostname;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate target_info;
extern crate xdg;

mod build_info;
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

fn main() {
    std::env::set_var("IN_QORK", "1");
    let context = Context::new();
    configure_logging(context.xdg());
    let _timer = ExecutionTimer::with_start_message("main.main");
    build_info::log_build_info();
    context.log_created_message();
    load_user_configuration(&context);
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

fn load_user_configuration(context: &Context) {
    let _timer = ExecutionTimer::with_start_message("main.load_user_configuration");

    if !context.program_info().parsed_args().load_config() {
        info!("Loading of user configuration is disabled.");
        return
    }

    let xdg = context.xdg();
    let dir = xdg.get_config_home();
    if !dir.exists() {
        warn!("The config_directory does not exist, no config will be loaded, config_directory={:?}", dir);
        return
    }

    if !dir.is_dir() {
        warn!("The config_directory is a file, not a directory, no config will be loaded, config_directory={:?}", dir);
        return
    }

    info!("Loading user configuration from {:?}", dir);

    let path = xdg.place_config_file("config.toml");
    match path {
        Ok(p) => {
            if p.exists() {

            } else {
                info!("The config file {:?} does not exist", p);
            }
        }
        Err(e) => {
            warn!("Error while asking for config.toml: {}", e);
            return;
        }
    }
}
