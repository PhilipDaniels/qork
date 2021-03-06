mod file_commands;

use buffer::BufferId;
use context::Context;
use commands::file_commands::*;

#[derive(Debug)]
pub enum Command {
    NoOp,
    Quit,
    OpenFile { filename: String },
    SaveBuffer { buffer_id: BufferId },
    SetCurrentBuffer { buffer_id: BufferId }
}

fn get_arg(line: &str) -> String {
    line.chars().skip(2).collect()
}

pub fn parse_command(line: &str) -> Command {
    if line == "q" {
        Command::Quit
    }
    else if line.starts_with("o ") {
        Command::OpenFile{ filename: get_arg(line) }
    }
    else if line.starts_with("s ") {
        let arg = get_arg(line);
        let id: BufferId = arg.parse().unwrap();
        Command::SaveBuffer{ buffer_id: id }
    }
    else if line.starts_with("c ") {
        let arg = get_arg(line);
        let id: BufferId = arg.parse().unwrap();
        Command::SetCurrentBuffer{ buffer_id: id }
    }
    else {
        Command::NoOp
    }
}

pub fn handle_command(context: &Context, command: Command) -> bool {
    match command {
        Command::NoOp => println!("No-op command"),
        Command::Quit => { println!("Quitting"); return true; }
        Command::OpenFile{filename} => handle_open_file(context, filename),
        Command::SaveBuffer{buffer_id} => handle_save_buffer(context, buffer_id),
        Command::SetCurrentBuffer{buffer_id} => handle_set_current_buffer(context, buffer_id)
    }

    false
}
