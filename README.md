# Lust

[![Build Status](https://travis-ci.org/SeanPrashad/Lust.svg?branch=master)](https://travis-ci.org/SeanPrashad/Lust)

## Description

A simple Rust library intended to retrieve information about your beloved file system!

## Functionality

Function Name | Purpose | Usage | Output
-- | - | - | -
`get_file_name(file_path: &String)`  | Displays a file name given the absolute path | `get_file_name("/home/FinalExam.txt")` | `"FinalExam.txt"`
`get_file_size(file_path: &String)`  | Displays the size of a file in bytes given the absolute path | `get_file_size("/home/FinalExam.txt")` | `1337 bytes`
`generate_sha(file_path: &String)` | Performs a SHA1 hash on the given file and displays the digest | `generate_sha("/home/FinalExam.txt")` |  `"2fd4e1c..."`
`generate_md5(file_path: &String)` | Performs a MD5 hash on the given file and displays the digest | `generate_md5("/home/FinalExam.txt")` | `"9e107d9..."`