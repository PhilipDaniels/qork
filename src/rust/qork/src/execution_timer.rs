use std::time::Instant;
use slog::Logger;

// When this struct is dropped, it logs a message stating its name and how long, in seconds,
// execution time was. Can be used to time functions or other critical areas.
pub struct ExecutionTimer<'a> {
	start_time: Instant,
	name: &'a str,
	logger: &'a Logger
}

impl<'a> ExecutionTimer<'a> {
	pub fn new(logger: &'a Logger, name: &'a str) -> ExecutionTimer<'a> {
		ExecutionTimer {
			start_time: Instant::now(),
			logger: logger,
			name: name
		}
	}

	// Construct a new ExecutionTimer and prints a message saying execution is starting.
	pub fn new2(logger: &'a Logger, name: &'a str) -> ExecutionTimer<'a> {
		debug!(logger, "Execution Started"; "Name" => name);
		ExecutionTimer::new(logger, name)
	}
}

impl<'a> Drop for ExecutionTimer<'a> {
	fn drop(&mut self) {
		let elapsed = self.start_time.elapsed();
        let secs = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
		debug!(self.logger, "Execution Completed"; "Seconds" => secs, "Name" => self.name);
	}
}
