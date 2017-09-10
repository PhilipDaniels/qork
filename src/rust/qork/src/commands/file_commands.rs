use std::path::PathBuf;
use context::Context;
use buffer::Buffer;
use utils;


pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();

    let mut bc = context.buffers();
    if let Some(_buffer) = bc.find_by_filename(&filename) {
        info!("Buffer for {} already exists.", &filename);
        return;
    }

    info!("Buffer for {} does not exist, creating new buffer.", &filename);
    let mut b = Buffer::new();
    b.filename = Some(PathBuf::from(&filename));
    context.state().mru().insert(filename);
    bc.add(b);
}
