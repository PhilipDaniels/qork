use context::Context;
use utils;

pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();

    let mut bc = context.buffers();
    if let Some(_buffer) = bc.find_by_filename(&filename) {
        info!("Buffer for {} already exists.", &filename);
        return;
    }

    info!("Buffer for {} does not exist, creating new buffer.", &filename);
    let mut fac = context.buffer_factory();
    let b = fac.open_file(&filename);
    bc.insert(b);
    context.state().mru().insert(filename);
}
