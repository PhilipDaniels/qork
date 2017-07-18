#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;
use slog::Logger;

struct LogTimer {
	name: String,
	log: &Logger
}

impl LogTimer {
	pub fn new(logger: &Logger, name: &str) -> LogTimer {
		LogTimer { log: logger, name: String::from(name) }
	}
}

impl Drop for LogTimer {
	fn drop(&mut self) {
		debug!(self.log, "Dropping {}", self.name);
	}
}

fn main() {
	let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
	let log = slog::Logger::root(drain, o!());

	//trace!(log, "hello world");
	//debug!(log, "hello world");
	//info!(log, "hello world");
	//warn!(log, "hello world");
	//error!(log, "hello world");
	//crit!(log, "hello world");

	let t = LogTimer::new(&log, "Main");
	other(&log);

}

fn other(log: &Logger) {
	let t2 = LogTimer::new(log, "Other");
}
