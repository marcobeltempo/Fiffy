# Lust

**Description**: A simple *Rust* *l*ibrary intended to retrieve information about your local file system!

**Usage**: To begin using _Lust_, please follow the steps outlined below:

1. Clone _Lust_ to your local workstation using `git clone https://github.com/SeanPrashad/Lust.git`

Function Name | Purpose | Usage | Output
-- | - | - | - 
`get_file_name(file_path: &String)`  | Displays a file name given the absolute path | `get_file_name("/home/dhpmuh/909SPD/FinalExam.exe")` | `"FinalExam.exe"`
`get_file_size(file_path: &String)`  | Displays the size of a file in bytes given the absolute path | `get_file_size("/home/dhpmuh/909SPD/FinalExam.exe")` | `1337`
`generate_sha(file_path: &String)` | Performs a SHA1 hash on the given file and displays the digest | `generate_sha("/home/dhpmuh/909SPD/FinalExam.exe")` |  `"2fd4e1c..."`
`generate_md5(file_path: &String)` | Performs a MD5 hash on the given file and displays the digest | `generate_md5("/home/dhpmuh/909SPD/FinalExam.exe")` | `"9e107d9..."`