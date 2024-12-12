use std::fs;

fn blink(stone_list: Vec<u64>) -> Vec<u64> {
    let mut new_stone_list = Vec::new();

    for i in 0..stone_list.len() {
        if stone_list[i] == 0 {
            new_stone_list.push(1);
        } else if stone_list[i].to_string().len() % 2 == 0 {
            let str_num = stone_list[i].to_string();
            let split_idx = str_num.len() / 2;

            let left_num = str_num[0..split_idx].parse::<u64>().unwrap();
            let right_num = str_num[split_idx..].parse::<u64>().unwrap();

            new_stone_list.push(left_num);
            new_stone_list.push(right_num);
        } else {
            new_stone_list.push(stone_list[i] * 2024);
        }
    }

    new_stone_list
}

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day11-test.txt";
    } else {
        filename = "inputs/day11.txt";
    }

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut stones: Vec<u64> = input.split_ascii_whitespace()
                                .collect::<Vec<&str>>()
                                .into_iter()
                                .map(|e| e.parse::<u64>().unwrap())
                                .collect();
    
    for _ in 1..=25 {
        stones = blink(stones.clone());
    }

    println!("{}", stones.len());
}
