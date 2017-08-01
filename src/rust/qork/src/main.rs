extern crate chrono;
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
mod qork;
mod system_info;

use log::LogLevelFilter;
use log4rs::config::{Appender, Config, Logger, Root};

use context::Context;
use execution_timer::ExecutionTimer;

fn main() {
    configure_logging();
    std::env::set_var("IN_QORK", "1");
    let _timer = ExecutionTimer::with_start_message("main.main");

    let context = Context::new();
    context.log_created_message();

    load_user_configuration_if_valid(&context);
}

fn load_user_configuration_if_valid(context: &Context) {
    if context.program_info().parsed_args().load_config() {
        let _timer = ExecutionTimer::with_start_message("main.load_user_configuration_if_valid");
        let dir = context.xdg().get_config_home();
        if !dir.exists() {
            warn!("The config_directory does not exist, no config will be loaded, config_directory={:?}", dir);
            return
        }

        if !dir.is_dir() {
            warn!("The config_directory is a file, not a directory, no config will be loaded, config_directory={:?}", dir);
            return
        }

        load_user_configuration(context);

    } else {
        info!("Loading of user configuration is disabled.");
    }
}

fn load_user_configuration(context: &Context) {
    let _timer = ExecutionTimer::with_start_message("main.load_user_configuration");
}

fn configure_logging() {
    use log4rs::append::console::ConsoleAppender;

    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LogLevelFilter::Debug))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();
}