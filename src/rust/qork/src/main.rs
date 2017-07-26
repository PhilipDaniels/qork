extern crate chrono;
extern crate clap;
extern crate hostname;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate target_info;
extern crate xdg;

mod command_line_arguments;
mod context;
mod datetime;
mod execution_timer;
mod program_info;
mod system_info;
mod qork;

use command_line_arguments::CommandLineArguments;
use context::Context;
use execution_timer::ExecutionTimer;
use slog::Logger;

fn main() {
    std::env::set_var("IN_QORK", "1");
    let throw_away_logger = create_root_logger();
    let _timer = ExecutionTimer::new(&throw_away_logger, "main.main");

    let args = CommandLineArguments::new();
    let main_logger = create_root_logger();
    info!(&main_logger, "{:?}", args);
    let context = Context::new(main_logger, args);
    context.log_created_message();

    load_user_configuration_if_valid(&context);
}

fn create_root_logger() -> Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // TODO: We want dynamic configuration here.
    slog::Logger::root(drain, o!())
}

fn load_user_configuration_if_valid(context: &Context) {
    if context.program_info.parsed_args().load_config() {
        let _timer = ExecutionTimer::new2(&context.logger(), "load_user_configuration");
        let dir = context.xdg().get_config_home();
        if !dir.exists() {
            warn!(&context.logger(), "The config_directory does not exist, no config will be loaded"; "config_directory" => %dir.display());
            return
        }

        if !dir.is_dir() {
            warn!(&context.logger(), "The config_directory is a file, not a directory, no config will be loaded"; "config_directory" => %dir.display());
            return
        }

        load_user_configuration(context);

    } else {
        info!(&context.logger(), "Loading of user configuration is disabled.");
    }
}

fn load_user_configuration(context: &Context) {
}
