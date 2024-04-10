use std::fs;
use std::collections::HashMap;

fn main () {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day2-test.txt";
    } else {
        filename = "inputs/day2.txt";
    }

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut frequency: HashMap<char, u32> = HashMap::new();
    let mut twos = 0;
    let mut threes = 0;

    for line in input.lines() {
        for c in line.chars() {
            let count = frequency.entry(c).or_insert(0);
            *count += 1;
        }

        let num_twos =  frequency.values().filter(|&v| *v == 2).count();
        let num_threes =  frequency.values().filter(|&v| *v == 3).count();

        if num_twos > 0 {
            // we do it like this to prevent double-counting
            twos += 1;
        }

        if num_threes > 0 {
            // we do it like this to prevent double-counting
            threes += 1;
        }

        frequency.clear();
    }

    println!("{}", twos * threes);
}