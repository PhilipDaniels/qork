extern crate chrono;
extern crate clap;
extern crate hostname;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

mod command_line_arguments;
mod context;
mod datetime;
mod execution_timer;
mod qork;

use command_line_arguments::CommandLineArguments;
use context::Context;
use execution_timer::ExecutionTimer;
use slog::Logger;

fn main() {
    std::env::set_var("IN_QORK", "1");
    let throw_away_logger = create_root_logger();
    let _timer = ExecutionTimer::new(&throw_away_logger, "Main.Start");

    let args = CommandLineArguments::new();
    let main_logger = create_root_logger();
    let context = Context::new(main_logger, args);
    context.log_created_message();
}

fn create_root_logger() -> Logger {
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // TODO: We want dynamic configuration here.
    slog::Logger::root(drain, o!())
}
