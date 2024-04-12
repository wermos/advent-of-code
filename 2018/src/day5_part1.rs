use std::fs;

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day5-test.txt";
    } else {
        filename = "inputs/day5.txt";
    }

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut s: Vec<char> = input.chars().collect();

    let mut i = 0;

    while i < s.len() - 1 {
        if s[i].is_uppercase() && s[i].to_ascii_lowercase() == s[i + 1] {
            s.remove(i);
            s.remove(i);

            if i > 0 {
                i -= 1;
            }
        } else if s[i].is_lowercase() && s[i].to_ascii_uppercase() == s[i + 1] {
            s.remove(i);
            s.remove(i);

            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    println!("{}", s.len());
}