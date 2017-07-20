#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate chrono;
extern crate hostname;

use slog::Logger;
mod execution_timer;
use execution_timer::ExecutionTimer;
mod context;
use context::Context;

fn main() {
    let logger = create_root_logger();
    let context = Context::new(logger);
    let _timer = ExecutionTimer::new(&context.logger, "Main.Start");
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
