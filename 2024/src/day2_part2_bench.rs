#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs;

fn convert(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect()
}

fn is_safe(list: &[i32]) -> bool {
    // we can safely assume that `list` has at least 2 elements.
    let increasing;

    if list[1] > list[0] {
        increasing = true;
    } else {
        increasing = false;
    };

    if increasing {
        for i in 1..list.len() {
            let diff = list[i] - list[i - 1];
    
            if diff <= 0 {
                // in this case, either the list had the same number twice in a
                // row, or started decreasing all of a sudden
                return false;
            }
    
            // we know that `diff` must be at least 1 by virtue of there being
            // no integer between 0 and 1.
            if diff > 3 {
                return false;
            }
        }
    } else {
        // then the list is decreasing
        for i in 1..list.len() {
            let diff = list[i - 1] - list[i];
    
            if diff <= 0 {
                // in this case, either the list had the same number twice in a
                // row, or started increasing all of a sudden
                return false;
            }
    
            // we know that `diff` must be at least 1 by virtue of there being
            // no integer between 0 and 1.
            if diff > 3 {
                return false;
            }
        }
    }
    true
}

#[bench]
fn two_vecs_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day2.txt";

        let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut _num_safe_lines = 0;

    for line in input.lines() {
        let list = convert(line);
        if is_safe(&list) {
            _num_safe_lines += 1;
        } else {
            for i in 0..list.len() {
                let mut tail = Vec::from(&list[i+1..]);
                let mut small_list = Vec::from(&list[..i]);
                
                small_list.append(&mut tail);
                
                if is_safe(&small_list) {
                    _num_safe_lines += 1;
                    break;
                }
            }
        }
    }
    });
}

#[bench]
fn vec_and_slice_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day2-test.txt";

        let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let mut _num_safe_lines = 0;

        for line in input.lines() {
            let list = convert(line);
            if is_safe(&list) {
                _num_safe_lines += 1;
            } else {
                for i in 0..list.len() {
                    let mut small_list = Vec::from(&list[..i]);
                    small_list.extend_from_slice(&list[i+1..]);
                    
                    if is_safe(&small_list) {
                        _num_safe_lines += 1;
                        break;
                    }
                }
            }
        }
    });
}

fn main() {
    println!("Run `cargo bench` to run the benchmarks.")
}