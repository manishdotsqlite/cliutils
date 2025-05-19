use clap::Parser;

use crate::implementation;

pub enum tools {
        echo(String),  // this repeats the string input
        cat(String, String), // this concatenates two files
        ls(String), // this lists directories of a folder
        find(String, String), // this locates files or directories in a directory and inside. 
        grep(String, String) // this matches text in files in a directory and inside.
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
        // this takes the command 
        #[arg()]
        pub command: String,

        // this takes the first string 
        #[arg()]
        first_argument: String,

        // this takes the second string
        #[arg(default_value_t = String::from(""))]
        second_argument: String,
}

pub struct Command {
        command: tools,
}

impl Command {
        pub fn init(arguments: Args) -> Result<Command, &'static str> {
                let first_argument = arguments.first_argument;
                let second_argument = arguments.second_argument;
                let command = match arguments.command.as_str() {
                        "echo" => tools::echo(first_argument),
                        "cat" => tools::cat(first_argument, second_argument),
                        "ls" => tools::ls(first_argument),
                        "find" => tools::find(first_argument, second_argument),
                        "grep" => tools::grep(first_argument, second_argument),
                        _ => return Err("Couldn't recognize command.")
                };

                let init_command = Command {command: command};
                Ok(init_command)    
        }

        pub fn execute(&self) -> Result<(), &'static str> {
                let _ = match &self.command {
                        tools::echo(input) =>  implementation::echo(input),
                        tools::cat(filename1, filename2) => implementation::cat(filename1, filename2)?,
                        tools::ls(directory) => implementation::ls(directory)?,
                        tools::find(directory, filename) => implementation::find(directory, filename)?,
                        tools::grep(directory, filename) => implementation::grep(directory, filename)?,
                };
            Ok(())
        }
}




