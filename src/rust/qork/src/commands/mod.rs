use context::Context;

#[derive(Debug)]
pub enum Command {
    NoOp,
    Quit,
    OpenFile { filename: String }
}

pub fn parse_command(line: &str) -> Command {
    if line == "q" {
        Command::Quit
    }
    else if line.starts_with("o ") {
        Command::OpenFile{ filename: line.chars().skip(2).collect() }
    }
    else {
        Command::NoOp
    }
}

pub fn handle_command(context: &Context, command: Command) -> bool {
    match command {
        Command::NoOp => { println!("Doing nothing"); }
        Command::Quit => { println!("Quitting"); return true; }
        Command::OpenFile{filename} => {
            println!("Opening file {}", filename);
            context.state().mru().insert(filename);
        }
    }

    false
}
