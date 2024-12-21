#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs;

use regex::Regex;

#[bench]
fn fold_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day3.txt";

        let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let re = Regex::new(r"mul\((?P<arg1>\d+),(?P<arg2>\d+)\)").unwrap();

        let _ans: i64 = re.captures_iter(input.as_str()).fold(0, |sum, m| {

            let arg1 = m["arg1"].parse::<i64>().unwrap();
            let arg2 = m["arg2"].parse::<i64>().unwrap();

            sum + arg1 * arg2
        });
    });
}

#[bench]
fn for_loop_version(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day3.txt";

        let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

        let re = Regex::new(r"mul\((?P<arg1>\d+),(?P<arg2>\d+)\)").unwrap();

        let mut _sum = 0;

        for m in re.captures_iter(input.as_str()) {    
            let arg1 = m["arg1"].parse::<i64>().unwrap();
            let arg2 = m["arg2"].parse::<i64>().unwrap();

            _sum += arg1 * arg2;
        }
    });
}


fn main() {
    println!("Run `cargo bench` to run the benchmarks.")
}