use std::fs;

fn mix(n: u64, val: u64) -> u64 {
    n ^ val
}

fn prune(n: u64) -> u64 {
    n % 16_777_216
}

fn gen_next(old: u64) -> u64 {
    let mut new = prune(mix(old, old * 64));

    new = prune(mix(new, new / 32));

    prune(mix(new, new * 2048))
}

fn gen_2000(n: u64) -> u64 {
    let mut new = gen_next(n);

    for _ in 2..=2000 {
        new = gen_next(new);
    }

    // println!("Start: {}\t2000th num: {}", n, new);
    new
}

fn main() {
    let filename = if cfg!(debug_assertions) {
        "inputs/day22-test.txt"
    } else {
        "inputs/day22.txt"
    };

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let nums: Vec<u64> = input.split("\n")
                              .map(|e| e.parse().unwrap())
                              .collect();

    let sum: u64 = nums.into_iter()
                       .map(|n| gen_2000(n))
                       .sum();

    println!("{}", sum);
}
