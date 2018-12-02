// Problem 2
#![allow(unused)]

extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::SeekFrom;
use std::io::Seek;


struct IdResult {
    has_two_of_letter: bool,
    has_three_of_letter: bool,
}


fn generate_id_result(s: String) -> IdResult {
    let mut m = HashMap::new();
    for c in s.chars() {
        if let Some(x) = m.get_mut(&c) {
            *x += 1u32;
        }
        else {
            m.insert(c, 1u32);
        }
    }

    let mut result = IdResult { has_two_of_letter: false,
                                has_three_of_letter: false };
    for val in m.values() {
        match val {
            2 => result.has_two_of_letter = true,
            3 => result.has_three_of_letter = true,
            _ => {},
        }
    }
    
    result
}


fn main() {
    let file_handle = File::open("resources/p2.txt").unwrap();
    let buffer_reader = BufReader::new(&file_handle);
    let mut result_listing = Vec::new();
    for line in BufReader::new(&file_handle).lines() {
        let l = line.unwrap();
        result_listing.push(generate_id_result(l));
    }

    let mut num2 = 0u32;
    let mut num3 = 0u32;
    for result in result_listing.iter() {
        if result.has_two_of_letter {
            num2 += 1u32;
        }
        if result.has_three_of_letter {
            num3 += 1u32;
        }
    }

    println!("Resulting Checksum {}", num2 * num3);
}
