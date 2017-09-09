use std::path::Path;
use context::Context;
use buffer::Buffer;
use utils;


pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();
    let p = Path::new(&filename);

    let mut buffers = context.buffers();
    match buffers.find_by_filename(&p) {
        Some(buffer) => buffer.is_changed = true,
        None => {}
    };

    match Buffer::open_file(p) {
        Some(buf) => {
            context.buffers().add(buf);
        //    context.state().mru().insert(filename)
        },
        None => {}
    }
}
