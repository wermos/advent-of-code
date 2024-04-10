use std::fs;

fn solve(filename: &str) -> i32 {
        let ans: i32 = fs::read_to_string(filename)
            .expect("Couldn't read file")
            .lines()
            .map(|line| line.parse().expect("Line is not a number!"))
            .fold(0, |total, i: i32| total + i);

        ans
}

fn main() {
    let filename = "inputs/day1.txt";

    let ans = solve(filename);

    println!("{ans}");
}