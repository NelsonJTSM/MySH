use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

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
struct ShellCommand<'a> {
    function: ShellFunction,
    name: &'a str,
    args: Vec<&'a str>,
}

#[derive(Debug)]
struct Shell {
    directory: PathBuf,
    history: Vec<String>,
}

// ShellState handles how the shell changes states.
impl Shell {
    fn new(directory: PathBuf) -> Shell {
        Shell {
            directory,
            history: Vec::new(),
        }
    }

    fn move_to_dir(&mut self, target_dir: &PathBuf) {
        self.directory = self.directory.join(target_dir);
    }

    fn where_am_i(&self) {
        println!("{}", self.directory.display());
    }

    fn add_to_history(&mut self, input: &str) {
        self.history.push(String::from(input));
    }

    fn print_history(&self) {
        for line in &self.history {
            println!("{}", line);
        }
    }

    fn start_program(&self, program_name: &str, program_args: Vec<&&str>) {
        let mut program = Command::new(program_name).args(program_args).spawn().unwrap();
        
        // println!("PID: {}", program.id());
        // drop(program);
        // program.wait().unwrap();
    }

    fn run_command(&mut self, command: ShellCommand, input: &str) {
        self.add_to_history(input);

        match command.function {
            ShellFunction::WhereAmI => self.where_am_i(),
            ShellFunction::MoveToDir => {
                let target_dir = PathBuf::from(command.args.get(0).unwrap());
                self.move_to_dir(&target_dir);
            },
            ShellFunction::ByeBye => {
                std::process::exit(0);
            },
            ShellFunction::History => {
                if command.args.len() == 0 {
                    self.print_history();
                }

                // Parse -c argument to clear history.
                match command.args.get(0) {
                    Some(arg) => {
                        if arg == &"-c" {
                            println!("Clearing history...");
                            self.history.clear();
                        }
                    },
                    None => {},
                }
            },
            ShellFunction::Start => {
                match command.args.get(0) {
                    Some(arg) => {
                        let program_args: Vec<&&str> = command.args.iter().skip(1).collect();
                        self.start_program(&arg, program_args);
                    },
                    None => {},
                }
            },
            _ => {
                // Better command handling?
                println!("Command not implemented yet.");
            },
        }
    }
}

fn main() {
    let prompt = "# ";

    let mut shell = start_shell_start();

    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                let command = interpret_command(&input.as_str());

                // TODO: Add the parsed input to the history, instead of just the trimmed input.
                shell.run_command(command, input.as_str().trim_end());
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn start_shell_start() -> Shell {
    // Assumes env::current_dir() is valid.
    let directory = std::env::current_dir().unwrap();

    Shell::new(directory)
}

fn interpret_command(input: &str) -> ShellCommand {
    let mut tokens = input.split_whitespace();

    // TODO: Handle error that occurs when no command is entered.
    // Code currently panics.
    let name: &str = tokens.next().unwrap().trim_end();
    let args: Vec<&str> = tokens.collect();
    let function = str_to_function(name);

    ShellCommand { function, args, name}
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
