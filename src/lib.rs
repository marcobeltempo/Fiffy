// Run program locally with: cargo build --features "clippy";

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate crypto;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsString;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use self::crypto::md5::Md5;

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

pub fn generate_sha1(file_path: &str) -> String {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!"
    );

    let mut sha_hash = Sha1::new();
    sha_hash.input_str(&file_contents.to_string());
    
    sha_hash.result_str()
}

pub fn generate_md5(file_path: &str) -> String {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!"
    );

    let mut md5_hash = Md5::new();
    md5_hash.input_str(&file_contents.to_string());
    
    md5_hash.result_str()
}

// To run tests, execute `cargo test`
#[cfg(test)]
mod tests {
    #[test]
    fn get_file_name() {
        use std::ffi::OsString;
        use ::get_file_name;

        let file_name = OsString::from("lazydog.txt");

        assert_eq!(get_file_name("/Desktop/lazydog.txt"), file_name);
        assert_eq!(get_file_name("/Desktop/lazydog.txt").len(), file_name.len());
    }

    #[test]
    fn get_file_size() {
        use ::get_file_size;

        let file_size : u64 = 43;

        assert_eq!(get_file_size("lazydog.txt"), file_size);
    }

    #[test]
    fn calc_sha1() {
        use ::generate_sha1;

        let sha1_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");

        assert_eq!(generate_sha1("lazydog.txt"), sha1_hash);
        assert_eq!(generate_sha1("lazydog.txt").len(), sha1_hash.len());
    }

    #[test]
    fn calc_md5() {
        use ::generate_md5;

        let md5_hash = String::from("9e107d9d372bb6826bd81d3542a419d6");

        assert_eq!(generate_md5("lazydog.txt"), md5_hash);
        assert_eq!(generate_md5("lazydog.txt").len(), md5_hash.len());;
    }
}