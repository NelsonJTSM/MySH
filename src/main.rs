use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ShellFunction {
    MoveToDir,
    WhereAmI,
    History,
    ByeBye,
    Start,
    Background,
    Exterminate,
    ExterminateAll,
    Repeat,
    UnknownFunction,
    NoFunction,
}

#[derive(Debug)]
struct ShellCommand<'a> {
    function: ShellFunction,
    name: &'a str,
    args: Vec<&'a str>,
}

impl<'a> ShellCommand<'a> {
    fn new(command: &str) -> ShellCommand {
        ShellCommand::interpret_command(command)
    }

    fn interpret_command(input: &str) -> ShellCommand {
        let mut tokens = input.split_whitespace();
        
        let name: &str = tokens.next().unwrap_or("");
        let args: Vec<&str> = tokens.collect();
        let function = ShellCommand::str_to_function(name);
        
        ShellCommand {
            function,
            args,
            name,
        }
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
            "background" => ShellFunction::Background,
            "exterminate" => ShellFunction::Exterminate,
            "exterminateall" => ShellFunction::ExterminateAll,
            "repeat" => ShellFunction::Repeat,
            _ => ShellFunction::UnknownFunction,
        }
    }
}

#[derive(Debug)]
struct Shell {
    directory: PathBuf,
    history: Vec<String>,
    pids: Vec<i32>,
}

// ShellState handles how the shell changes states.
impl Shell {
    fn new(directory: PathBuf) -> Shell {
        Shell {
            directory,
            history: Vec::new(),
            pids: Vec::new(),
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

    fn start_program(&mut self, program_name: &str, program_args: Vec<&&str>) {
        let mut program = Command::new(program_name)
            .args(program_args)
            .spawn()
            .unwrap();

        program.wait().unwrap();
        self.pids.push(program.id() as i32);
    }

    fn background_program(&mut self, program_name: &str, program_args: Vec<&&str>) {
        let program = Command::new(program_name)
            .args(program_args)
            .spawn()
            .unwrap();
        self.pids.push(program.id() as i32);

        println!("PID: {}", program.id());
    }

    fn exterminate_all(&mut self) {
        for pid in &self.pids {
            self.exterminate_program(*pid);
        }

        self.pids.clear();
    }

    fn exterminate_program(&self, pid: i32) {
        unsafe {
            libc::kill(pid, libc::SIGKILL);
        };
    }

    fn run_command(&mut self, command: ShellCommand, input: &str) {
        if !input.is_empty() {
            self.add_to_history(input);
        }

        match command.function {
            ShellFunction::WhereAmI => self.where_am_i(),
            ShellFunction::MoveToDir => {
                let target_dir = PathBuf::from(command.args.get(0).unwrap());
                self.move_to_dir(&target_dir);
            }
            ShellFunction::ByeBye => {
                std::process::exit(0);
            }
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
                    }
                    None => {}
                }
            }
            ShellFunction::Start => match command.args.get(0) {
                Some(arg) => {
                    let program_args: Vec<&&str> = command.args.iter().skip(1).collect();
                    self.start_program(&arg, program_args);
                }
                None => {}
            },
            ShellFunction::Exterminate => match command.args.get(0) {
                Some(arg) => {
                    let pid = String::from(*arg).parse::<i32>().unwrap();
                    self.exterminate_program(pid);
                }
                None => {
                    println!("Argument required for {}", command.name);
                }
            },
            ShellFunction::Background => match command.args.get(0) {
                Some(arg) => {
                    let program_args: Vec<&&str> = command.args.iter().skip(1).collect();
                    self.background_program(&arg, program_args);
                }
                None => {}
            },
            ShellFunction::ExterminateAll => {
                self.exterminate_all();
            }
            ShellFunction::Repeat => match (command.args.get(0), command.args.get(1)) {
                (Some(arg), Some(cmd)) => {
                    let n = String::from(*arg).parse::<u32>().unwrap();
                    let function = ShellCommand::str_to_function(*cmd);
                    // TODO: Handle arguments better.
                    // Right now I am cloning like crazy,
                    // reducing the efficiency.
                    let mut new_args = command.args.clone();
                    new_args.remove(0);
                    new_args.remove(0);

                    for _ in 0..n {
                        let shell_command = ShellCommand {
                            function: function,
                            name: *cmd,
                            args: new_args.clone(),
                        };

                        self.run_command(shell_command, *cmd);
                    }
                }
                (_, _) => {
                    println!("Incorrect use of repeat.");
                }
            },
            _ => {
                // Better command handling?
                println!("Command not implemented yet.");
            }
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
                let command = ShellCommand::new(&input.as_str());

                // TODO: Add the parsed input to the history, instead of just the trimmed input.
                shell.run_command(command, input.as_str().trim_end());
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn start_shell_start() -> Shell {
    let directory = std::env::current_dir().unwrap_or(PathBuf::default());

    Shell::new(directory)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_parsing() {
        let input_output = vec![
            ("", ShellFunction::NoFunction),
            ("movetodir", ShellFunction::MoveToDir),
            ("repeat", ShellFunction::Repeat),
            ("repeat n start /usr/bin/alacritty", ShellFunction::Repeat),
            ("kjdsljf", ShellFunction::UnknownFunction),
            ("\n\n\n\n", ShellFunction::NoFunction),
            ("\n\n\nmovetodir\n\n\n", ShellFunction::MoveToDir),
            ("        repeat n      \n", ShellFunction::Repeat),
        ];

        for (input, expected_function) in input_output {
            let command = ShellCommand::new(input);
            assert_eq!(command.function, expected_function);
        }
    }
}
