// Run program locally with `cargo build --features "clippy";`

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate sha1;
extern crate md5;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ffi::OsString;

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

pub fn generate_md5(file_path: &str) -> md5::Digest {
    let mut file = File::open(file_path.trim()).expect("File Not Found!");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect(
        "Something went wrong reading the file!"
    );

    md5::compute(file_contents)
}

// To run tests, execute `cargo test` whilst inside the repo file structure
#[cfg(test)]
pub mod tests {
    #[test]
    fn get_file_name() {

    }

    #[test]
    fn get_file_size() {
        
    }

    #[test]
    fn calc_sha1() {
        use sha1;

        let mut m = sha1::Sha1::new();

        m.reset();
        m.update("The quick brown ".as_bytes());
        m.update("fox jumps over ".as_bytes());
        m.update("the lazy dog".as_bytes());
        let hh = m.digest().to_string();

        let h = "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12";
        assert_eq!(hh.len(), h.len());
        assert_eq!(hh, &*h);
    }

    #[test]
    fn calc_md5() {
    }
}