#![feature(test)]

extern crate test;

use test::Bencher;

use std::fs;

#[bench]
fn way1(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day1.txt";

        let input = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut total = 0;

        for line in input.lines() {
            let number: i32 = line.parse().unwrap();
            total += number;
        }
        
        total
    });
}

#[bench]
fn way2(b: &mut Bencher) {
    b.iter(|| {
        let filename = "inputs/day1.txt";

        let ans: i32 = fs::read_to_string(filename)
            .expect("Couldn't read file")
            .lines()
            .map(|line| line.parse().expect("Line is not a number!"))
            .fold(0, |total, i: i32| total + i);

        ans
    });
}

fn main() {
    println!("Run `cargo bench` to run the benchmarks.")
}