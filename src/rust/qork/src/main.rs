#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

use slog::Drain;

fn main() {
	let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
	let log = slog::Logger::root(drain, o!());

	trace!(log, "hello world");
	debug!(log, "hello world");
	info!(log, "hello world");
	warn!(log, "hello world");
	error!(log, "hello world");
	crit!(log, "hello world");
}
