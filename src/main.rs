extern crate sha1; // For use of SHA1 functions
extern crate md5; // For use of MD5 functions

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude:: * ;
use std::io;

/*
    To run program: 
        `cargo run` [File_Name_Goes_Here_As_Command_Line_Argument]
         ex: cargo run lazydog.txt
         
        `cargo run` Prompt user for file name

        q OR Q to exit the program    
*/

fn main() {
  let args: Vec < String > = env::args().collect();

  // File I/O: https://doc.rust-lang.org/book/second-edition/ch12-02-reading-a-file.html
  //checks if user passed in a file name through the command line
  if args.len() > 1 {
    file_processor( & args[1]);
  }

  let mut quit: bool = false;

  // Continue loop until user exits program
  while !quit {

    io::stdout().flush().unwrap();

    print!("\nEnter a valid file path | quit(q|Q): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin().read_line( & mut input).ok().expect(
      "Couldn't read line",
    );

    //validate_file(&String) returns the current state
    quit = validate_file( & input);
  }
}

fn validate_file(file_path: & String) -> bool {

//checks if file name entered by the user is valid
  let is_valid = Path::new(file_path.trim()).exists();

  //exits program if user enters (q|Q)
  if file_path.trim().to_uppercase() == "Q" {
    return true;
  }

  if !is_valid {
    println!("Error: '{}' was not found", file_path.trim());
  } else {
    //if the file is valid, begin to process the data
    file_processor(file_path);
  }
  return false;
}

fn file_processor(file_path: & String) {

  get_file_name(file_path);
  get_file_size(file_path);
  generate_sha(file_path);
  generate_md5(file_path);
}

fn get_file_name(file_path: & String) {
  let path = Path::new(file_path.trim());
  println!("\nFile name: {:?}", path.file_name().unwrap());
}

/*
    [21:42:20]  <wyvern>    Depending on the sort of code you’re writing, you could display a nice error message, or get user input for a better file path to look at, or just crash if you’re writing a test or a throwaway experiment
*/

fn get_file_size(file_path: & String) -> () {
  match fs::metadata(file_path.trim()) {
    Ok(metadata) => {
      println!("File size: {:?} bytes", metadata.len());
    }
    Err(e) => {
      println!("Error fetching file metadata! {:?}", e);
    }
  }
}

fn generate_sha(file_path: & String) -> () {
  let mut file = File::open(file_path.trim()).expect("File Not Found!");

  let mut file_contents = String::new();
  file.read_to_string( & mut file_contents).expect(
    "Something went wrong reading the file!",
  );

  let mut sha_hash = sha1::Sha1::new();
  sha_hash.update(file_contents.as_bytes());

  println!("SHA1 hash: {:?}", sha_hash.digest().to_string());
}

fn generate_md5(file_path: & String) -> () {
  let mut file = File::open(file_path.trim()).expect("File Not Found!");

  let mut file_contents = String::new();
  file.read_to_string( & mut file_contents).expect(
    "Something went wrong reading the file!",
  );

  let md5 = md5::compute(file_contents);

  println!("MD5 hash: \"{:x}\"", md5);
}