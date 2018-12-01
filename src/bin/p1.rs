// Problem 1

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let fileHandle = File::open("resources/p1.txt").unwrap();
    let bufReader = BufReader::new(&fileHandle);
    for line in bufReader.lines() {
        println!("{}\t", line.unwrap());
    }
}
