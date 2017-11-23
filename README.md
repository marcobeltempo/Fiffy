# Lust

[![Build Status](https://travis-ci.org/SeanPrashad/Lust.svg?branch=master)](https://travis-ci.org/SeanPrashad/Lust)

## Description

A simple Rust library intended to retrieve information about your local file system!

## Usage

To begin using _Lust_, please follow the steps outlined below:

1; Clone _Lust_ to your local workstation using Terminal or Git Bash: `git clone https://github.com/SeanPrashad/Lust.git`
2; Navigate into the root level of the folder; ex: `cd ~/repos/Lust`
3; Build the `Cargo` project using: `cargo build`
4; Run the project whilst supplying a **single command-line argument** (optional) of the **absolute path** for a file you'd like to diagnose; ex: `cargo run lazydog.txt` (where `lazydog.txt` is in the root level folder of our `Cargo` project)
5; Marvel in awe at the results!
6; You'll be prompted for another file or enter `q` OR `Q` to exit

## Functionality

Function Name | Purpose | Usage | Output
-- | - | - | -
`get_file_name(file_path: &String)`  | Displays a file name given the absolute path | `get_file_name("/home/dhpmuh/909SPD/FinalExam.exe")` | `"FinalExam.exe"`
`get_file_size(file_path: &String)`  | Displays the size of a file in bytes given the absolute path | `get_file_size("/home/dhpmuh/909SPD/FinalExam.exe")` | `1337 bytes`
`generate_sha(file_path: &String)` | Performs a SHA1 hash on the given file and displays the digest | `generate_sha("/home/dhpmuh/909SPD/FinalExam.exe")` |  `"2fd4e1c..."`
`generate_md5(file_path: &String)` | Performs a MD5 hash on the given file and displays the digest | `generate_md5("/home/dhpmuh/909SPD/FinalExam.exe")` | `"9e107d9..."`