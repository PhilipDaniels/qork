use std::path::Path;
use context::Context;
use buffer::Buffer;

pub fn handle_open_file(context: &Context, filename: String) {
    match Buffer::open_file(Path::new(&filename)) {
        Some(buf) => context.state().mru().insert(filename),
        None => {}
    }
}
