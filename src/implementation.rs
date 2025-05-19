use std::fs::{self, File, read_dir, metadata};
use std::io::{Write};
use std::path::Path;



fn echo(input: &str) {
        println!("{:?}", input);
}

fn cat(filename1: &str, filename2: &str) -> Result<(), &'static str> {
        let path1 = Path::new(filename1);
        let path2 = Path::new(filename2);

        let contents1 = match fs::read_to_string(path1) {
                Ok(some) => some,
                Err(_) => return Err("Error reading first file.")
        };

        let contents2 = match fs::read_to_string(path2) {
                Ok(some) => some,
                Err(_) => return Err("Error reading first file.")
        };

        let newfilename = "concatenated-".to_owned() + filename1 + "-" + filename2;
        let mut newfile = match File::create(newfilename) {
                Ok(some) => some,
                Err(_) => return Err("Can't created new file.")
        };

        let binding = contents1 + &contents2 ;
        let buffer = binding.as_bytes();
        match newfile.write_all(buffer) {
                Ok(_) => Ok(()),
                Err(_) => Err("Error when creating new file."),
        }

}


fn ls(directory: &str) -> Result<(), &'static str>{
        let path = Path::new(directory);

        if !path.exists() {
                return Err("Path doesn't exist.")
        }

        let entries = fs::read_dir(directory);
       
        Ok(())
}