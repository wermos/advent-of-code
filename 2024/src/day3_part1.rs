use std::fs;
use regex::Regex;

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day3-part1-test.txt";
    } else {
        filename = "inputs/day3.txt";
    }

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let re = Regex::new(r"mul\((?P<arg1>\d+),(?P<arg2>\d+)\)").unwrap();

    let mut sum = 0;

    for m in re.captures_iter(input.as_str()) {
        // println!("{:?}", m);

        let arg1 = m["arg1"].parse::<i64>().unwrap();
        let arg2 = m["arg2"].parse::<i64>().unwrap();

        // println!("{:?} and {:?}", arg1, arg2);

        sum += arg1 * arg2;
    }

    println!("{sum}");
}
