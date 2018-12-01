// Problem 1
extern crate regex;

use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let file_handle = File::open("resources/p1.txt").unwrap();
    let buffer_reader = BufReader::new(&file_handle);

    let mut twice = None;
    let mut frequency_hit = HashSet::new();
    let mut total = 0;
    for line in buffer_reader.lines() {
        let l = line.unwrap();
        let value = l.parse::<i32>().unwrap();
        total += value;
        if frequency_hit.contains(&total) && twice == None {
            twice = Some(total.clone());
        }
        else {
            frequency_hit.insert(total.clone());
        }
    }

    println!("The total frequency: {}", total);
    println!("First Frequency hit Twice: {}", twice.unwrap());
}
