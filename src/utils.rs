use clap::Parser;

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
        command: String,

        // this takes the first string 
        #[arg()]
        first_argument: String,

        // this takes the second string
        #[arg(default_value_t = String::from(""))]
        second_argument: String,
}




