use std::fs;

use common::cpu::CPU;

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day19-test.txt";
    } else {
        filename = "inputs/day19.txt";
    }

    let mut cpu = CPU::part1();

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // Create an iterator over the lines
    let mut lines = input.lines();

    // Consume the first line
    if let Some(first_line) = lines.next() {
        let val = first_line.split_ascii_whitespace().last().unwrap();

        let val: usize = val.parse().unwrap();
        cpu.set_ip_reg(val);
    }

    let program: Vec<&str> = lines.collect();
    cpu.execute(program);
    println!("{}", cpu.read_reg_0());

}
