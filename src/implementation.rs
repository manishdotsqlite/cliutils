use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;



pub fn echo(input: &str) {
        println!("{:?}", input);
}

pub fn cat(filename1: &str, filename2: &str) -> Result<(), &'static str> {
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

        let vec1: Vec<&str> =  filename1.split('/').collect();
        let vec2: Vec<&str> =  filename2.split('/').collect();

        let actual_filename1 = match vec1.last() {
                Some(some) => some,
                None => return Err("Error reading filename.")
        };

        let actual_filename2 = match vec2.last() {
                Some(some) => some,
                None => return Err("Error reading filename.")
        };

        let newfilename = "src/test/concatenated-".to_owned() + actual_filename1 + "-" + actual_filename2 + ".txt";
        println!("New file name: {:?}", newfilename);
        let mut newfile = match File::create(newfilename) {
                Ok(some) => some,
                Err(_) => return Err("Can't create new file.")
        };

        let binding = contents1 + &contents2 ;
        let buffer = binding.as_bytes();
        match newfile.write_all(buffer) {
                Ok(_) => Ok(()),
                Err(_) => Err("Error when creating new file."),
        }

}


pub fn ls(directory: &str) -> Result<(), &'static str>{
        let path = Path::new(directory);
        if !path.exists() {
                return Err("Path doesn't exist.")
        }

        let entries = match fs::read_dir(directory){
                Ok(some) => some,
                Err(_) => return Err("Couldn't read directory.")
        };

        for entry in entries {
                match entry {
                        Ok(some) => {
                                let path = some.path();
                                match fs::metadata(&path) {
                                        Ok(some) => {
                                                if some.is_dir() {
                                                        if let Some(s) = path.file_name() {
                                                                let file_name_string = s.to_string_lossy().into_owned();
                                                                println!("Folder: {:?}", file_name_string);
                                                        }       
                                                } else if some.is_file() {
                                                        if let Some(s) = path.file_name() {
                                                                let file_name_string = s.to_string_lossy().into_owned();
                                                                println!("File: {:?}", file_name_string);
                                                        }
                                                } else {
                                                        println!("Couldn't read file.")
                                                }
                                        },
                                        Err(_) => println!("Couldn't read metadata.")
                                        
                                };

                        },
                        Err(_) => ()
                }
        }
       
        Ok(())
}


pub fn find(directory: &str, filename: &str) -> Result<(), &'static str> {
        let initial_path = Path::new(directory);
        if !initial_path.exists() {
                return Err("Path doesn't exist.")
        }

        let entries = match fs::read_dir(initial_path) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't read directory.")
        };

        for entry in entries {
                match entry {
                        Ok(individual_entry) => {
                                let path = individual_entry.path();
                                match fs::metadata(&path) {
                                        Ok(some) => {
                                                if some.is_file() {
                                                        if let Some(s) = path.file_name() {
                                                                let file_name_string = s.to_string_lossy().into_owned();
                                                                if file_name_string == filename {
                                                                        println!("File found at: {:?}", path.display());
                                                                }
                                                        }                                        
                                                } else if some.is_dir() {
                                                        let new_path = path.to_string_lossy().into_owned();
                                                        let _ = find(&new_path, filename);
                                                } else {
                                                        println!("Can't read entry.")
                                                }
                                        },
                                        Err(_) => ()
                                }
                        },
                        Err(_) => ()
                }
        }

        Ok(())
}


pub fn grep(directory: &str, text: &str) -> Result<() , &'static str> {
        let initial_path = Path::new(directory);
        if !initial_path.exists() {
                return Err("Path doesn't exist.")
        }

        let entries = match fs::read_dir(initial_path) {
                Ok(some) => some,
                Err(_) => return Err("Couldn't read directory.")
        };

        fn check_file(filename: &str, text_chunk: &str) -> Result<(), &'static str> {
                let file = match File::open(filename) {
                        Ok(some) => some, 
                        Err(_) => return Err("Couldn't open file.")
                };

                let reader = BufReader::new(file);

                for  (line_number, line_result) in reader.lines().enumerate() {
                        match line_result {
                                Ok(line) => {
                                        
                                        if line.contains(text_chunk) {
                                                println!(" {:?} text found at file {:?}, at line {:?}", text_chunk, filename, line_number+1);
                                        }
                                },
                                Err(_) => ()
                        }; 
                }

                Ok(())
        }

        for entry in entries {
                match entry {
                        Ok(some) => {
                                let path = some.path();
                                match fs::metadata(&path) {
                                        Ok(some) => {
                                                if some.is_file() {
                                                        if let Some(s) = path.file_name() {
                                                                let newpath = path.to_string_lossy().into_owned();
                                                                let _ = check_file(&newpath, text);
                                                        }
                                                } else if some.is_dir() {
                                                        let new_path = path.to_string_lossy().into_owned();
                                                        let _ = grep(&new_path, text);
                                                }
                                        },
                                        Err(_) => ()
                                }
                        },
                        Err(_) => ()
                }
        }

        Ok(())
}
