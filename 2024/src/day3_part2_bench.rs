#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs;

use regex::Regex;

#[bench]
fn single_regex_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day3.txt";

        let raw_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let input = raw_input.as_str();

        // `re` is used to find the initial matches
        let re = Regex::new(r"don't\(\)|do\(\)|mul\((?P<arg1>\d+),(?P<arg2>\d+)\)").unwrap();
        
        let matches = re.captures_iter(input);
        let mut active = true;

        let mut _sum = 0;

        for cap in matches {
            match cap.get(0).unwrap().as_str() {
                "do()" => active = true,
                "don't()" => active = false,
                // that means we matched on a `mul`
                _ => if active {
                    // run the regex match function again, but this time we capture the extra,
                    // necessary info, in capture groups
                    let arg1 = cap["arg1"].parse::<i64>().unwrap();
                    let arg2 = cap["arg2"].parse::<i64>().unwrap();

                    _sum += arg1 * arg2;
                },
            }
        }
    });
}

#[bench]
fn two_regex_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day3.txt";

        let raw_input = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let input = raw_input.as_str();

        // `re` is used to find the initial matches
        let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d+,\d+\)").unwrap();
        // `mul_re` is used in the main loop. We define it out here to prevent recompilation
        // of the regex string at every iteration.
        let mul_re = Regex::new(r"mul\((?P<arg1>\d+),(?P<arg2>\d+)\)").unwrap();

        let matches = re.find_iter(input);
        let mut active = true;

        let mut _sum = 0;

        for m in matches {
            match m.as_str() {
                "do()" => active = true,
                "don't()" => active = false,
                // that means we matched on a `mul`
                _ => if active {
                    // run the regex match function again, but this time we capture the extra,
                    // necessary info, in capture groups

                    let m = mul_re.captures_at(input, m.start()).unwrap();
                    let arg1 = m["arg1"].parse::<i64>().unwrap();
                    let arg2 = m["arg2"].parse::<i64>().unwrap();

                    _sum += arg1 * arg2;
                },
            }
        }
    });
}


fn main() {
    println!("Run `cargo bench` to run the benchmarks.")
}