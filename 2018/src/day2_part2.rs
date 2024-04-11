use std::fs;

fn compare(s1: String, s2: String) -> Option<String> {
    let mut diff = 0;

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
        }
    }

    if diff == 1 {
        let common: String = s1.chars().zip(s2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect();

        Some(common)
    } else {
        None
    }
}

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day2-test-part2.txt";
    } else {
        filename = "inputs/day2.txt";
    }

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let lines = input.lines();

    for word1 in lines.clone() {
        for word2 in lines.clone() {
            if word1 == word2 {
                continue;
            }

            if let Some(common) = compare(word1.to_string(), word2.to_string()) {
                // println!("{:?} {:?}", word1, word2);
                println!("{}", common);
                return;
            }
        }
    }
}