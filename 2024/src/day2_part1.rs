use std::fs;

fn convert(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|e| e.parse::<i32>().unwrap()).collect()
}

fn is_safe(list: &[i32]) -> bool {
    // we can safely assume that `list` has at least 2 elements.
    let increasing = if list[1] > list[0] {
        true
    } else {
        false
    };

    if increasing {
        for i in 1..list.len() {
            let diff = list[i] - list[i - 1];
    
            if diff <= 0 {
                return false;
            }
    
            if diff > 3 {
                return false;
            }
        }
    } else {
        // then the list is decreasing
        for i in 1..list.len() {
            let diff = list[i - 1] - list[i];
    
            if diff <= 0 {
                return false;
            }

            if diff > 3 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let filename = if cfg!(debug_assertions) {
        "inputs/day2-test.txt"
    } else {
        "inputs/day2.txt"
    };

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut num_safe_lines = 0;

    for line in input.lines() {
        if is_safe(&convert(line)) {
            num_safe_lines += 1;
        }
    }

    println!("{num_safe_lines}");
}
