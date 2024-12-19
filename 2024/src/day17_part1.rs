use std::fs;

struct CPU {
    registers: [usize; 3],
    ip: usize,
    first_output: bool,
    output: String,
}

impl CPU {
    pub fn new(a: usize, b: usize, c: usize) -> Self {
        Self {
            registers: [a, b, c],
            ip: 0,
            first_output: true,
            output: String::new(),
        }
    }

    fn incr_ip(&mut self) {
        self.ip += 1;
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Unexpected operand."),

        }
    }

    // A division (perform division and then store in A)
    fn adv(&mut self, operand: usize) {
        if operand <= 3 {
            self.registers[0] /= 2_usize.pow(operand as u32);
        } else {
            self.registers[0] /= 2_usize.pow(self.combo(operand) as u32);
        }
    }

    // bitwise xor
    fn bxl(&mut self, operand: usize) {
        self.registers[1] ^= operand;
    }

    // mod 8
    fn bst(&mut self, operand: usize) {
        if operand <= 3 {
            // operand is between 0-3 (inclusive) so operand % 8 = operand.
            self.registers[1] = operand;
        } else {
            self.registers[1] = self.combo(operand) % 8;
        }
    }

    // jump if not zero
    fn jnz(&mut self, operand: usize) {
        if self.registers[0] != 0 {
            self.ip = operand;        
        } else {
            self.incr_ip();
        }
    }

    // bitwise xor, again
    fn bxc(&mut self, _operand: usize) {
        self.registers[1] ^= self.registers[2];
    }

    // mod 8, but output this time
    fn out(&mut self, operand: usize) {
        let output = if operand <= 3 {
            // operand is between 0-3 (inclusive) so operand % 8 = operand.
            operand
        } else {
            self.combo(operand) % 8
        };

        if self.first_output {
            self.output.push_str(format!("{output}").as_str());
            self.first_output = false;
        } else {
            self.output.push_str(format!(",{output}").as_str());
        }
    }

    // B division, (perform division and then store in B)
    fn bdv(&mut self, operand: usize) {
        if operand <= 3 {
            self.registers[1] = self.registers[0] / 2_usize.pow(operand as u32);
        } else {
            self.registers[1] = self.registers[0] / 2_usize.pow(self.combo(operand) as u32);
        }
    }

    // C division, (perform division and then store in C)
    fn cdv(&mut self, operand: usize) {
        if operand <= 3 {
            self.registers[2] = self.registers[0] / 2_usize.pow(operand as u32);
        } else {
            self.registers[2] = self.registers[0] / 2_usize.pow(self.combo(operand) as u32);
        }
    }

    pub fn execute(&mut self, program: Vec<(usize, usize)>) {
        while self.ip < program.len() {
            // while ip is pointing to a valid instruction

            let (instr, operand) = program[self.ip];
            // println!("Instruction: {}, Operand: {}\t Registers: {:?}", instr, operand, self.registers);

            match instr {
                0 => {
                    self.adv(operand);
                },
                1 => {
                    self.bxl(operand);
                },

                2 => {
                    self.bst(operand);
                },
                3 => {
                    self.jnz(operand);
                }

                4 => {
                    self.bxc(operand);
                },
                5 => {
                    self.out(operand);
                },
                6 => {
                    self.bdv(operand);
                },
                7 => {
                    self.cdv(operand);
                },

                _ => println!("Unexpected instruction received"),
            }

            if instr != 3 {
                self.incr_ip();
                // jump instruction doesn't need the IP increment
            }
        }
        println!("{}", self.output);
    }
}

fn main() {
    let filename;

    if cfg!(debug_assertions) {
        filename = "inputs/day17-test.txt";
    } else {
        filename = "inputs/day17.txt";
    }

    let input = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    let mut it = input.lines();

    let a = it.next().unwrap()
                     .split(" ")
                     .last().unwrap()
                     .parse::<usize>().unwrap();
    let b = it.next().unwrap()
                     .split(" ")
                     .last().unwrap()
                     .parse::<usize>().unwrap();
    let c = it.next().unwrap()
                     .split(" ")
                     .last().unwrap()
                     .parse::<usize>().unwrap();

    let mut cpu = CPU::new(a, b, c);

    // take the last whitespace-split component of the last line.
    let program_str = it.last().unwrap()
                              .split_whitespace()
                              .last().unwrap();

    let tmp: Vec<&str> = program_str.split(",").collect();

    let program: Vec<(usize, usize)> = tmp.chunks(2)
        .map(|slice| {
            (slice[0].parse().unwrap(), slice[1].parse().unwrap())
        })
        .collect();

    // println!("Program: {program_str}");
    // println!("{program:?}");

    cpu.execute(program);
}