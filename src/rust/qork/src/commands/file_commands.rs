use buffer::BufferFactory;
use context::Context;
use utils;

pub fn handle_open_file(context: &Context, filename: String) {
    let mut fac = BufferFactory::new();

    let filename = utils::expand_variables(&filename).to_string();

    let mut bc = context.buffers();
    if let Some(_buffer) = bc.find_by_filename(&filename) {
        info!("Buffer for {} already exists.", &filename);
        return;
    }

    info!("Buffer for {} does not exist, creating new buffer.", &filename);
    match fac.open_file(&filename) {
        Some(buffer) => {
            bc.add(buffer);
            context.state().mru().insert(filename);
        },
        None => {}
    }
}
