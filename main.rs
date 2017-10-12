use std::path::Path;
use std::ffi::OsString;

fn main(){
    let path = Path::new("/home/dhpmuh/909SPD/FinalExam.exe");

    // Optional values - refer to: https://doc.rust-lang.org/std/option/ 
    println!("1. {:?}", getFileName(path)); // unwrap() supresses the "Some" keyword
}

// Unlike C/C++, there's NO restrictions on the order of function definitions!

/*
    getFileName(Path fileName) -> OsString
    OsString: https://doc.rust-lang.org/std/ffi/struct.OsString.html
 */
fn getFileName(filePath: &Path) -> OsString {
    return OsString::from(filePath.file_name().unwrap());
}   