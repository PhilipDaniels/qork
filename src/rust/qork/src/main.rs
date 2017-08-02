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
