**Description**

Test if two files have the same contents.


**Syntax**

same _file1_ _file2_
Prints "equal" if the contents of _file1_ and contents of _file2_ are equal, or else it prints "not equal".


**Compilation**

1: Ensure you have some version of the Rust toolchain with the command:
rustup -V

2: cd into the directory to which you have cloned this repository, enter the directory also called "same", and use the following command:
cargo build
 to produce a binary at ./target/debug/same .
