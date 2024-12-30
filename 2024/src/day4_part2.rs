use std::fs;

fn num_hits(input: Vec<Vec<char>>) -> u64 {
    let mut hits = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            if input[i][j] == 'A' {
                // we check left-to-right first
                if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S') ||
                   (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M') {
                    // now we check right-to-left
                    if (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S') ||
                       (input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M') {
                        hits += 1;
                    }
                }
            }
        }
    }

    hits
}

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day4-test.txt";
    } else {
        filename = "inputs/day4.txt";
    }

    let input = fs::read_to_string(filename)
                           .expect("Something went wrong reading the file");
    
    let input: Vec<Vec<char>> = input.lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .into_iter()
        .map(|s| s.chars().collect()).collect();

    println!("{}", num_hits(input));
}
