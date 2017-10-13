extern crate sha1;  // For use of SHA1 functions
extern crate md5;   // For use of MD5 functions

use std::env;
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
    generate_sha(&args[1]);
    generate_md5(&args[1]);
}

fn get_file_name(file_path: &String) -> () {
    let path = Path::new(file_path);
    println!("File name: {:?}", path.file_name().unwrap());
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

/*
    getFileSize(Path file_path) -> OsString
 */
//fn get_file_size(file_path: &Path) -> fs::Metadata {
    // let metadata = fs::metadata(file_path);
    //return OsString::from(metadata);
    //return fs::metadata(filePath).unwrap();

    // let metadata = fs::metadata("foo.txt");
    // //assert!(!metadata.is_dir());
    // return metadata.file_type.unwrap();

    //assert_eq!(0, metadata.len());
//}