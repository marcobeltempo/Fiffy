extern crate sha1;  // For use of SHA1 functions
extern crate md5;   // For use of MD5 functions

use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

/*
    To run program: cargo run [File_Name_Goes_Here_As_Command_Line_Argument]
        ex: cargo run lazydog.txt
*/

fn main(){
    let args: Vec<String> = env::args().collect();

    // File I/O: https://doc.rust-lang.org/book/second-edition/ch12-02-reading-a-file.html
    get_file_name(&args[1]);
    get_file_size(&args[1]);
    generate_sha(&args[1]);
    generate_md5(&args[1]);
}

fn get_file_name(file_path: &String) -> () {
    let path = Path::new(file_path);
    println!("File name: {:?}", path.file_name().unwrap());
}

/*
    [21:42:20]  <wyvern>    Depending on the sort of code you’re writing, you could display a nice error message, or get user input for a better file path to look at, or just crash if you’re writing a test or a throwaway experiment
*/

fn get_file_size(file_path: &String) -> () {
    match fs::metadata(file_path)
    { 
        Ok(metadata) => { println!("File size: {:?} bytes", metadata.len()); },
        Err(e) => { println!("Error fetching file metadata! {:?}", e); } 
    }
}

fn generate_sha(file_path: &String) -> () {
    let mut file = File::open(file_path).expect("File Not Found!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Something went wrong reading the file!");

    let mut sha_hash = sha1::Sha1::new();
    sha_hash.update(file_contents.as_bytes());

    println!("SHA1 hash: {:?}", sha_hash.digest().to_string());
}

fn generate_md5(file_path: &String) -> () {
    let mut file = File::open(file_path).expect("File Not Found!");

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("Something went wrong reading the file!");

    let md5 = md5::compute(file_contents);

    println!("MD5 hash: \"{:x}\"", md5);
}