use context::Context;
use utils;

pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();

    let mut bc = context.buffers();
    if let Some(_buffer) = bc.find_by_filename(&filename) {
        info!("Buffer for {} already exists.", &filename);
        return;
    }

    let mut fac = context.buffer_factory();
    let b = fac.open_file(&filename);
    info!("Buffer for {} does not exist, creating new buffer with id of {}", &filename, b.id());
    bc.insert(b);
    context.state().mru().insert(filename);
}

pub fn handle_save_buffer(context: &Context, buffer_id: u64) {
    match context.buffers().get(buffer_id) {
        Some(rc) => {
            let buffer = rc.borrow_mut();
            info!("Saving buffer with id of {}", buffer.id());
        },
        None => { warn!("No buffer with an id of {} exists", buffer_id) }
    }
}
