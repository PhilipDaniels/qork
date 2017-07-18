#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use std::time::{Duration, Instant};
use std::thread;
use slog::Drain;
use slog::Logger;

struct LogTimer<'a> {
	start_time: Instant,
	name: &'a str,
	logger: &'a Logger
}

impl<'a> LogTimer<'a> {
	pub fn new(logger: &'a Logger, name: &'a str) -> LogTimer<'a> {
		LogTimer {
			start_time: Instant::now(),
			logger: logger,
			name: name
		}
	}
}

impl<'a> Drop for LogTimer<'a> {
	fn drop(&mut self) {
		let elapsed = self.start_time.elapsed();
        let secs = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
		debug!(self.logger, "Execution Completed"; "Seconds" => secs, "Name" => self.name);
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

	let _lt1 = LogTimer::new(&log, "main");
	other(&log);

	let s = "another".to_string();
	thread::sleep(Duration::from_millis(400));
	let _lt2 = LogTimer::new(&log, &s);
}

fn other(log: &Logger) {
	let _lt = LogTimer::new(log, "other");
}
