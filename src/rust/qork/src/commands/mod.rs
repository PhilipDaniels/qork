mod file_commands;

use context::Context;
use commands::file_commands::*;

#[derive(Debug)]
pub enum Command {
    NoOp,
    Quit,
    OpenFile { filename: String },
    SaveBuffer { buffer_id: u64 },
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
        let id: u64 = arg.parse().unwrap();
        Command::SaveBuffer{ buffer_id: id }
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
        Command::SaveBuffer{buffer_id} => handle_save_buffer(context, buffer_id)
    }

    false
}
