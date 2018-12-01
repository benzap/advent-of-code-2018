// Problem 1
extern crate regex;

use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn get_first_frequency_double() -> i32 {
    let mut total = 0;
    let mut frequency_hit = HashSet::new();
    while(true) {
        let file_handle = File::open("resources/p1.txt").unwrap();
        let buffer_reader = BufReader::new(&file_handle);
        for line in buffer_reader.lines() {
            let l = line.unwrap();
            let value = l.parse::<i32>().unwrap();
            total += value;

            if (frequency_hit.contains(&total)) {
                return total;
            }
            else {
                frequency_hit.insert(value.clone());
            }
        }
    }
    total
}

fn main() {
    let file_handle = File::open("resources/p1.txt").unwrap();
    let buffer_reader = BufReader::new(&file_handle);


    let mut total = 0;
    for line in buffer_reader.lines() {
        let l = line.unwrap();
        let value = l.parse::<i32>().unwrap();
        total += value;
    }

    println!("The total frequency: {}", total);
    println!("First Frequency hit Twice: {}", get_first_frequency_double());
}
