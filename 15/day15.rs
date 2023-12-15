#![feature(extract_if)]
use std::io::{self, BufRead};
use std::collections::LinkedList;
use std::str::FromStr;

// tilt so that all 'O' characters want to DECREMENT their Y index
// using custom `at` function that "remaps" x and y, this function can be used for all directions
fn ascii_hash(s: &str) -> usize {
    let mut hash: usize = 0;
    for c in s.as_bytes().iter() {
        hash += *c as usize;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

fn print(boxes: &Vec<LinkedList<(String, u8)>>) {
    for (i, l) in boxes.iter().enumerate() {
        if l.is_empty() { continue;}
        print!("Box {}: ", i);
        for t in l.iter() {
            print!("[{} {}] ", t.0, t.1);
        }
        println!();
    }
}


fn main() {
    // let input = "example.txt";
    let input = "input.txt";
    let mut lines = read_lines(&input);
    let mut hash_sum: usize = 0;
    let mut boxes = vec![LinkedList::<(String, u8)>::new(); 256];
    for s in lines.next().expect("Line not ok").unwrap().split(',') {
        hash_sum += ascii_hash(&s) as usize;
        if *s.as_bytes().last().unwrap() == b'-' {
            let hash = ascii_hash(&s[..s.len()-1]);
            let _ = boxes[hash].extract_if(|t| t.0 == &s[..s.len()-1]).for_each(drop);
        }
        else { 
            let hash = ascii_hash(&s[..s.len()-2]);
            let focal_length = s[s.len()-1..].parse::<u8>().unwrap();
            if let Some(t) = boxes[hash].iter_mut().filter(|t| t.0 == &s[..s.len()-2]).next() {
                t.1 = focal_length; 
            }
            else {
                boxes[hash].push_back((String::from_str(&s[..s.len()-2]).expect("String err"), focal_length));
            }
        }
        // println!("\nAfter \"{}\"", s);
        // print(&boxes);
    }
    println!("Hash sum (1): {}", hash_sum);
    println!("Focus power sum (2): {}", boxes.iter().enumerate().map(|(b, l)| l.iter().enumerate().map(|(i, t)| (b+1) * (i+1) * t.1 as usize).sum::<usize>()).sum::<usize>());
}

fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<std::fs::File>>
where P: AsRef<std::path::Path>, {
    return match std::fs::File::open(filename) {
        Err(why) => panic!("Could not open file. {}", why),
        Ok(file) => std::io::BufReader::new(file).lines()
    };
}

