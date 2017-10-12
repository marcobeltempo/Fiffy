extern crate sha1;

use std::path::Path;    // For use in getFileName()
use std::ffi::OsString; // For use in getFileName()

fn main(){
    let path = Path::new("foo.txt");

    // Optional values - refer to: https://doc.rust-lang.org/std/option/ 
    println!("1. {:?}", get_file_name(path)); // unwrap() supresses the "Some" keyword
    //println!("2. {:?}", get_file_size(path));



    let m = sha1::Sha1::new();

    let s = String::from(m.digest().to_string());

    println!("{:?}", s);

    //let mut m = sha1::Sha1::new();
    //m.update(b"Hello World!");
    // println!("3. {:?}", generateSHA(path));

    //assert_eq!(m.digest().to_string(), "2ef7bde608ce5404e97d5f042f95f89f1c232871");
}

// fn generateSHA(file_path: &Path) -> ! {
//     let mut m = sha1::Sha1::new();

//     let s = String::from(m);

//     //m.update(OsString::from(file_path.file_name()));
// }

// Unlike C/C++, there's NO restrictions on the order of function definitions!

/*
    getFileName(Path file_path) -> OsString
    OsString: https://doc.rust-lang.org/std/ffi/struct.OsString.html
 */
fn get_file_name(file_path: &Path) -> OsString {
    return OsString::from(file_path.file_name().unwrap());
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