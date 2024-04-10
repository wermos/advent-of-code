use std::fs;
use std::collections::HashSet;

fn main() {
    let filename = String::from("inputs/day1.txt");

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut values: Vec<i32> = vec![];

    for line in input.lines() {
        values.push(line.parse().unwrap());
    }

    let mut frequencies = HashSet::new();

    let mut current_frequency = 0;

    let mut counter = 0;
    
    loop {    
        current_frequency += values[counter];

        if frequencies.contains(&current_frequency) {
            println!("{}", current_frequency);
            break;
        }

        frequencies.insert(current_frequency);

        counter += 1;

        if counter == values.len() {
            counter = 0;
        }
    }
}