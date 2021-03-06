use std::time::Instant;

// When this struct is dropped, it logs a message stating its name and how long, in seconds,
// execution time was. Can be used to time functions or other critical areas.
pub struct ExecutionTimer<'a> {
	start_time: Instant,
	name: &'a str
}

impl<'a> ExecutionTimer<'a> {
	pub fn new(name: &'a str) -> ExecutionTimer<'a> {
		ExecutionTimer {
			start_time: Instant::now(),
			name: name
		}
	}

	// Construct a new ExecutionTimer and prints a message saying execution is starting.
	pub fn with_start_message(name: &'a str) -> ExecutionTimer<'a> {
		debug!("Execution Starting, Name={}", name);
		ExecutionTimer {
			start_time: Instant::now(),
			name: name
		}
	}
}

impl<'a> Drop for ExecutionTimer<'a> {
	fn drop(&mut self) {
		let elapsed = self.start_time.elapsed();
        let secs = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
        let msecs = secs * 1000.0;
		debug!("Execution Completed, Name={}, MilliSecs={:.3}", self.name, msecs);
	}
}

#[macro_use]
mod macros {
    /// Creates a timer that logs a starting and completed message.
    #[macro_export]
    macro_rules! timer {
        ($str:expr) => { ::execution_timer::ExecutionTimer::with_start_message($str) }
    }

    /// Creates a quiet timer that does not log a starting message, only a completed one.
    #[macro_export]
    macro_rules! quiet_timer {
        ($str:expr) => { ::execution_timer::ExecutionTimer::new($str) }
    }
}
