use clap::Parser;

enum tools {
        echo(String),  // this repeats the string input
        cat(String, String), // this concatenates two files
        ls(String), // this lists directories of a folder
        find(String), // this locates files or directories in root directory and inside. 
        grep(String) // this matches text in files in the root directory and inside.
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
        // this takes the command 
        #[arg()]
        command: String,

        // this takes the first string 
        #[arg()]
        first_argument: String,

        // this takes the second string
        #[arg()]
        second_argument: String,
}