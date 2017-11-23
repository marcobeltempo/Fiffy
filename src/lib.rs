// Run program locally with `cargo build --features "clippy";`

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate sha1;
extern crate md5;

use std::fs;
use std::str;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsString;
use std::ops::Deref;

pub fn get_file_name(file_path: &str) -> OsString {
    let path = Path::new(file_path.trim());
    path.file_name().unwrap().to_os_string()
}

pub fn get_file_size(file_path: &str) -> u64 {
    match fs::metadata(file_path.trim()) {
        Ok(metadata) => {
            metadata.len()
        }
        Err(_e) => {
            0
        }
    }
}

pub fn generate_sha1(file_path: &str) -> sha1::Sha1 {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!"
    );

    let mut sha_hash = sha1::Sha1::new();
    sha_hash.update(file_contents.as_bytes());

    sha_hash
}

pub fn generate_md5(file_path: &str) -> &str {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!"
    );

    let x = md5::compute(file_contents);

    println!("Calculated MD5 digest is: {:?}", x.deref());

    "placeholder"
}

// To run tests, execute `cargo test` whilst inside the repo file structure
#[cfg(test)]
pub mod tests {
    #[test]
    fn get_file_name() {
        use std::ffi::OsString;
        use ::get_file_name;

        let file_name = OsString::from("lazydog.txt");

        assert_eq!(get_file_name("./lazydog.txt"), file_name);
    }

    #[test]
    fn get_file_size() {
        use ::get_file_size;

        let file_size : u64 = 43;

        assert_eq!(get_file_size("./lazydog.txt"), file_size);
    }

    #[test]
    fn calc_sha1() {
        // use sha1;

        // let mut m = sha1::Sha1::new();

        // m.reset();
        // m.update("The quick brown ".as_bytes());
        // m.update("fox jumps over ".as_bytes());
        // m.update("the lazy dog".as_bytes());

        // // Create a string using my functions and assert that below!

        // let hh = m.digest().to_string();

        // let h = "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12";
        // assert_eq!(hh.len(), h.len());
        // assert_eq!(hh, &*h);
    }

    #[test]
    fn calc_md5() {
        // use ::generate_md5;

        // let string_val = String::from("lazydog.txt");

        // assert_eq!(generate_md5(&string_val), "c3fcd3d76192e4007dfb496cca67e13b");
    }
}