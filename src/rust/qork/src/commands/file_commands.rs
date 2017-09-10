use std::path::Path;
use context::Context;
use buffer::Buffer;
use utils;


pub fn handle_open_file(context: &Context, filename: String) {
    let filename = utils::expand_variables(&filename).to_string();

    let bc = context.buffers();
    match bc.find_by_filename(&filename) {
        Some(buffer) => { info!("Buffer for {} already exists.", &filename); },
        None => {
           info!("Buffer for {} does not exist, creating new buffer.", &filename);
           let mut bc = context.buffers();
           let b = Buffer::new();
           context.state().mru().insert(filename);
           bc.add(b);
        }
    }
    // if let Some(buffer) = .find_by_filename(&filename) {
    //     info!("Buffer for {} already exists.", filename);
    // } else {
    //     info!("Buffer for {} does not exist, creating new buffer.", filename);
    //     let mut b = Buffer::new();
    //     let mut b2 = context.buffers();
    //     b2.add(b);
    //     context.state().mru().insert(filename);
    // }

    // let mut buffers = context.buffers();
    // let buf = buffers.find_by_filename(&filename);
    // if buf.is_none() {
    //     info!("Buffer for {} does not exist, creating new buffer.", filename);
    //     let mut b = Buffer::new();
    //     buffers.add(b);
    //     context.state().mru().insert(filename);
    // } else {
    //     info!("Buffer for {} already exists.", filename);
    // }
}
