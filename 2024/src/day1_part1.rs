use std::fs;

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day1-test.txt";
    } else {
        filename = "inputs/day1.txt";
    }

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let temp: Vec<&str> = line.split_whitespace().collect();

        v1.push(temp[0].parse::<i32>().unwrap());
        v2.push(temp[1].parse::<i32>().unwrap());
    }

    // println!("{:?}\n\n{:?}", v1, v2);

    v1.sort();
    v2.sort();

    let mut sum: i32 = 0;

    // we know that the two vectors are of the same length
    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).abs();
    }

    println!("{:?}", sum);
}
