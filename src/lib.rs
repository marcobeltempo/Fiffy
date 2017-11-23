#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate sha1;
extern crate md5;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsString;

fn get_file_name(file_path: &str) -> OsString {
    let path = Path::new(file_path.trim());
    path.file_name().unwrap().to_os_string()
}

<<<<<<< HEAD
=======
// IO functions may fail which returns a Result<T, err> that we must handle
>>>>>>> b4654b49675b9b5cd935feeec0dbcc864e80aeb4
fn get_file_size(file_path: &str) -> u64 {
    match fs::metadata(file_path.trim()) {
        Ok(metadata) => {
            metadata.len()
        }
        Err(_e) => {
            0
        }
    }
}

fn generate_sha(file_path: &str) -> sha1::Sha1 {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!",
    );

    let mut sha_hash = sha1::Sha1::new();
    sha_hash.update(file_contents.as_bytes());

    sha_hash
}

fn generate_md5(file_path: &str) -> md5::Digest {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!",
    );

    md5::compute(file_contents)
}