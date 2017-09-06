use std::path::Path;
use context::Context;
use buffer::Buffer;
use utils;

pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();
    match Buffer::open_file(Path::new(&filename)) {
        Some(buf) => {
            context.buffers().add(buf);
            context.state().mru().insert(filename)
        },
        None => {}
    }
}
