# Groovy Git

**Description**: A simple Rust library containing the following four functions:

Function Name | Purpose | Usage | Output
-- | - | - | - 
`getFileName(string filePath)`  | Consumes a string representing the absolute file path for a single file, returning only the file name | `getFileName("/home/dhpmuh/909SPD/FinalExam.exe")` | `"FinalExam.exe"`
`getFileSize(string filePath)`  | Consumes a string representing the absolute file path for a single file, returning the file size in bytes | `getFileSize("/home/dhpmuh/909SPD/FinalExam.exe")` | `128`
`generateSHA(string filePath)` | Consumes a string representing the absolute file path for a single file, returning the calculated SHA1 hash | `generateSHA("The quick brown fox jumps over the lazy dog")` |  `"2fd4e1c..."`
`generateMD5(string filePath)` | Consumes a string representing the absolute file path for a single file, returning the calculated MD5 hash | `generateMD5("The quick brown fox jumps over the lazy dog")` | `"9e107d9..."`
