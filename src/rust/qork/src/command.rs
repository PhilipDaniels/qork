#[derive(Debug)]
pub enum Command {
    NoOp,
    Quit,
    OpenFile { filename: String }
}
