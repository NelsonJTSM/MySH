use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
enum ShellFunction {
    MoveToDir,
    WhereAmI,
    History,
    ByeBye,
    Start,
    UnknownFunction,
    NoFunction,
}

#[derive(Debug)]
struct Command<'a> {
    function: ShellFunction,
    args: Vec<&'a str>,
}

#[derive(Debug)]
struct ShellState {
    directory: PathBuf,
}

// ShellState handles how the shell changes states.
impl ShellState {
    fn current_directory(&mut self) -> &mut PathBuf {
        // Assumes directory doesn't have weird chars.
        &mut self.directory
    }

    fn where_am_i(&self) {
        println!("{}", self.directory.display());
    }
}

fn main() {
    let prompt = "# ";

    let mut shell_state = start_shell_start();

    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            let command = interpret_command(&input.as_str());
            run_command(command, shell_state);
        }
        Err(error) => println!("error: {}", error),
    }
}

fn start_shell_start() -> ShellState {
    // Assumes env::current_dir() is valid.
    let directory = std::env::current_dir().unwrap();

    ShellState {directory}
}

fn run_command(command: Command, mut state: ShellState) {
    match command.function {
        ShellFunction::WhereAmI => {state.where_am_i()},
        _ => {},
    }
}

fn interpret_command(input: &str) -> Command {
    let mut tokens = input.split_whitespace();

    // TODO: Handle error that occurs when no command is entered.
    // Code currently panics.
    let cmd: &str = tokens.next().unwrap().trim_end();
    let args: Vec<&str> = tokens.collect();
    let function = str_to_function(cmd);

    Command {function, args}
}

fn str_to_function(command: &str) -> ShellFunction {
    // TODO: Ignore lowercase for commands.

    match command {
        "" => ShellFunction::NoFunction,
        "movetodir" => ShellFunction::MoveToDir,
        "whereami" => ShellFunction::WhereAmI,
        "history" => ShellFunction::History,
        "byebye" => ShellFunction::ByeBye,
        "start" => ShellFunction::Start,
        _ => ShellFunction::UnknownFunction,
    }
}