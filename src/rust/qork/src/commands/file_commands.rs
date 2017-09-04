use context::Context;

pub fn handle_open_file(context: &Context, filename: String) {
    println!("Opening file {}", filename);
    context.state().mru().insert(filename);
}
