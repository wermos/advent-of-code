use std::fs;

fn simplify(mut s: Vec<char>) -> Vec<char> {
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

    s
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day5-test.txt";
    } else {
        filename = "inputs/day5.txt";
    }

    let input = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let s: Vec<char> = simplify(input.chars().collect());

    let ascii_lowercase = 'a'..='z';
    let mut lengths = [0; 26];

    for ch in ascii_lowercase {
        let mut s = s.clone();

        if s.contains(&ch) {
            s.retain(|&c| c != ch && c != ch.to_ascii_uppercase());

            lengths[ch as usize - 'a' as usize] = simplify(s).len();
        } else {
            lengths[ch as usize - 'a' as usize] = s.len();
        }
    }

    println!("{}", lengths.iter().min().unwrap());
    
    let elapsed = now.elapsed();
    println!("Took {:.2?}.", elapsed);
}