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


fn generate_id_result(s: &String) -> IdResult {
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


fn get_id_listing() -> Vec<String> {
    let file_handle = File::open("resources/p2.txt").unwrap();
    let buffer_reader = BufReader::new(&file_handle);
    let mut id_listing = Vec::new();
    for line in BufReader::new(&file_handle).lines() {
        let l = line.unwrap();
        id_listing.push(l);
    }
    id_listing
}


fn generate_diff_complement(id_listing: &Vec<String>,
                            id_test: &String,
                            id_index: usize) -> String {
    let mut best_diff = String::new();
    for (index, id) in id_listing.iter().enumerate() {
        if id_index == index { continue; }
        let mut cdiff = String::new();
        for i in 0..id.len() {
            let a = id_test.chars().collect::<Vec<char>>()[i];
            let b = id.chars().collect::<Vec<char>>()[i];
            if a == b {
                cdiff.push(a);
            }
        }
        
        if best_diff.len() < cdiff.len() {
            best_diff = cdiff.clone();
        }
    }
    best_diff
}


fn get_best_box_id(id_listing: &Vec<String>) -> String {
    let mut best_cdiff = String::new();
    for (index, id) in id_listing.iter().enumerate() {
        let check_cdiff = generate_diff_complement(id_listing, id, index);
        if best_cdiff.len() < check_cdiff.len() {
            best_cdiff = check_cdiff.clone();
        }
    }
    best_cdiff
}


fn main() {
    let id_listing = get_id_listing();
    let mut result_listing = Vec::new();
    for id in &id_listing {
        result_listing.push(generate_id_result(id));
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

    println!("Resulting Checksum: {}", num2 * num3);
    println!("Best ID Diff Complement: {}", get_best_box_id(&id_listing));
}
