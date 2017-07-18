#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;
use slog::Logger;

mod log_timer;
use log_timer::LogTimer;

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
	let _lt2 = LogTimer::new(&log, &s);
}

fn other(log: &Logger) {
	let _lt = LogTimer::new(log, "other");

	{
		let _lt2 = LogTimer::new(log, "other2");
		sleep(200);
	}

	{
		let _lt3 = LogTimer::new2(log, "other3");
		sleep(300);
	}
}


fn sleep(msec: u64) {
	use std::thread;
	use std::time::Duration;
	thread::sleep(Duration::from_millis(msec));
}
