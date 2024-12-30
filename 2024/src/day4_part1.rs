use std::fs;

fn num_hits(input: Vec<Vec<char>>) -> u64 {
    let mut hits = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'X' {
                if j < input[i].len() - 3 {
                    // check forwards
                    if input[i][j + 1] == 'M' && input[i][j + 2] == 'A' && input[i][j + 3] == 'S' {
                        hits += 1;
                    }
                }

                if j >= 3 {
                    // check backwards
                    if input[i][j - 3] == 'S' && input[i][j - 2] == 'A' && input[i][j - 1] == 'M' {
                        hits += 1;
                    }
                }

                if i >= 3 {
                    // check vertical
                    if input[i - 1][j] == 'M' && input[i - 2][j] == 'A' && input[i - 3][j] == 'S' {
                        hits += 1;
                    }

                    if j >= 3 {
                        // check backwards (upward) diagonal
                        if input[i - 1][j - 1] == 'M' && input[i - 2][j - 2] == 'A' && input[i - 3][j - 3] == 'S' {
                            hits += 1;
                        }
                    }

                    if j < input[i].len() - 3 {
                        // check forward (upward) diagonal
                        if input[i - 1][j + 1] == 'M' && input[i - 2][j + 2] == 'A' && input[i - 3][j + 3] == 'S' {
                            hits += 1;
                        }
                    }
                }

                if i < input.len() - 3 {
                    // check vertical
                    if input[i + 1][j] == 'M' && input[i + 2][j] == 'A' && input[i + 3][j] == 'S' {
                        hits += 1;
                    }

                    if j >= 3 {
                        // check backwards (downward) diagonal
                        if input[i + 1][j - 1] == 'M' && input[i + 2][j - 2] == 'A' && input[i + 3][j - 3] == 'S' {
                            hits += 1;
                        }
                    }

                    if j < input[i].len() - 3 {
                        // check forward (downwards) diagonal
                        if input[i + 1][j + 1] == 'M' && input[i + 2][j + 2] == 'A' && input[i + 3][j + 3] == 'S' {
                            hits += 1;
                        }
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
