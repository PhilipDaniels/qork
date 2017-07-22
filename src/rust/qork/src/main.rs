#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate chrono;
extern crate hostname;
extern crate clap;

use slog::Logger;
mod execution_timer;
use execution_timer::ExecutionTimer;
mod context;
use context::Context;
mod command_line_arguments;
use command_line_arguments::CommandLineArguments;

// TODO
// * Why has my app ballooned to 18MB in size? How can I tell how large a crate is?
// * Move version to a 'constants' file.
// * Tidy up the context: make fields private.

fn main() {
    let throw_away_logger = create_root_logger();
    let _timer = ExecutionTimer::new(&throw_away_logger, "Main.Start");

    let args = CommandLineArguments::new();
    let main_logger = create_root_logger();
    let context = Context::new(main_logger, args);
    context.log_created_message();
}

fn create_root_logger() -> Logger {
    // trace, debug, info, warn, error, crit.
    use slog::Drain;

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    // TODO: We want dynamic configuration here.
    slog::Logger::root(drain, o!())
}


/*
fn sleep(msec: u64) {
    use std::thread;
    use std::time::Duration;
    thread::sleep(Duration::from_millis(msec));
}
*/
