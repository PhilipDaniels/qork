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
    let id = b.id();
    info!("Buffer for {} does not exist, creating new buffer with id of {}", &filename, b.id());
    bc.insert(b);
    bc.set_current_buffer(id);
    context.state().mru().insert(filename);
}

pub fn handle_save_buffer(context: &Context, buffer_id: u64) {
    match context.buffers().get(buffer_id) {
        Some(rc) => {
            let buffer = rc.borrow_mut();
            info!("Saving buffer with id of {} and filename of {:?}", buffer.id(), buffer.filename());
        },
        None => { warn!("No buffer with an id of {} exists", buffer_id) }
    }
}

pub fn handle_set_current_buffer(context: &Context, buffer_id: u64) {
    let mut bc = context.buffers();
    match bc.set_current_buffer(buffer_id) {
        true => info!("Current buffer changed to {}", buffer_id),
        false => info!("The buffer {} does not exist", buffer_id)
    }
}