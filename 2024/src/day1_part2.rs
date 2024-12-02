use std::fs;
use std::collections::HashMap;

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day1-test.txt";
    } else {
        filename = "inputs/day1.txt";
    }

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    // even though this approach will require two passes, it's simpler so we opt
    // for this. We make a the two lists of numbers, and initialize the hashmap
    // using the elements of the first vector.
    for line in input.lines() {
        let temp: Vec<&str> = line.split_whitespace().collect();

        v1.push(temp[0].parse::<i32>().unwrap());
        v2.push(temp[1].parse::<i32>().unwrap());
        map.entry(*v1.last().unwrap()).or_insert(0);
    }

    for elem in v2.iter() {
        if map.contains_key(elem) {
            *map.get_mut(elem).unwrap() += 1;
        }
    }

    let mut sum: i32 = 0;

    // we know that the two vectors are of the same length
    for elem in v1.iter() {
        sum += elem * map[elem];
    }

    println!("{:?}", sum);
}
