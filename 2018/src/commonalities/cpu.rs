pub struct CPU {
    registers: [i32; 6],
    ip_reg_idx: usize,
}

impl CPU {
    pub fn part1() -> Self {
        CPU {
            registers: [0; 6],
            ip_reg_idx: 0,
        }
    }

    pub fn part2() -> Self {
        CPU {
            registers: [1, 0, 0, 0, 0, 0],
            ip_reg_idx: 0,
        }
    }

    // ip stuff
    pub fn set_ip_reg(&mut self, reg_idx: usize) {
        self.ip_reg_idx = reg_idx;
    }

    fn incr_ip(&mut self) {
        self.registers[self.ip_reg_idx] += 1;
    }

    fn read_ip(&self) -> usize {
        self.registers[self.ip_reg_idx] as usize
    }

    // answer
    pub fn read_reg_0(&self) -> i32 {
        self.registers[0]
    }

    // add impls
    fn addr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] + self.registers[b];
    }

    fn addi(&mut self, a: usize, b: i32, c: usize) {
        self.registers[c] = self.registers[a] + b;
    }

    // mul impls
    fn mulr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] * self.registers[b];
    }

    fn muli(&mut self, a: usize, b: i32, c: usize) {
        self.registers[c] = self.registers[a] * b;
    }

    // bitwise AND
    fn banr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] & self.registers[b];
    }

    fn bani(&mut self, a: usize, b: i32, c: usize) {
        self.registers[c] = self.registers[a] & b;
    }

    // bitwise OR
    fn borr(&mut self, a: usize, b: usize, c: usize) {
        self.registers[c] = self.registers[a] | self.registers[b];
    }

    fn bori(&mut self, a: usize, b: i32, c: usize) {
        self.registers[c] = self.registers[a] | b;
    }

    // assignment
    fn setr(&mut self, a: usize, c: usize) {
        self.registers[c] = self.registers[a];
    }

    fn seti(&mut self, a: i32, c: usize) {
        self.registers[c] = a;
    }

    // greater-than testing
    fn gtir(&mut self, a: i32, b: usize, c: usize) {
        if a > self.registers[b] {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    fn gtri(&mut self, a: usize, b: i32, c: usize) {
        if self.registers[a] > b {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    fn gtrr(&mut self, a: usize, b: usize, c: usize) {
        if self.registers[a] > self.registers[b] {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    // equality testing
    fn eqir(&mut self, a: i32, b: usize, c: usize) {
        if a == self.registers[b] {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    fn eqri(&mut self, a: usize, b: i32, c: usize) {
        if self.registers[a] == b {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    fn eqrr(&mut self, a: usize, b: usize, c: usize) {
        if self.registers[a] == self.registers[b] {
            self.registers[c] = 1;
        } else {
            self.registers[c] = 0;
        }
    }

    pub fn execute(&mut self, program: Vec<&str>) {
        while self.read_ip() < program.len() - 1 {
            // while ip is pointing to a valid instruction

            // Split the input string by whitespace
            let mut parts = program[self.read_ip()].split_whitespace();
            println!("Executing instruction: {}\t Reg. 0 = {} \t Reg. 3 = {}", program[self.read_ip()], self.registers[0], self.registers[3]);

            if let (Some(inst), Some(a), Some(b), Some(c)) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            {
                match inst {
                    "addr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.addr(a, b, c);
                    },
                    "addi" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.addi(a, b, c);
                    },

                    "mulr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.mulr(a, b, c);
                    },
                    "muli" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.muli(a, b, c);
                    }

                    "banr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.banr(a, b, c);
                    },
                    "bani" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.bani(a, b, c);
                    },
                    "borr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.borr(a, b, c);
                    },
                    "bori" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.bori(a, b, c);
                    },

                    "setr" => {
                        let a = a.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.setr(a, c);
                    },
                    "seti" => {
                        let a = a.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.seti(a, c);
                    },

                    "gtir" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.gtir(a, b, c);
                    },
                    "gtri" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.gtri(a, b, c);
                    },
                    "gtrr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.gtrr(a, b, c);
                    },

                    "eqir" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.eqir(a, b, c);
                    },
                    "eqri" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.eqri(a, b, c);
                    },
                    "eqrr" => {
                        let a = a.parse().unwrap();
                        let b = b.parse().unwrap();
                        let c = c.parse().unwrap();
                        self.eqrr(a, b, c);
                    },
                    _ => println!("Unexpected instruction received"),
                }

                self.incr_ip();
            }
        }
    }
}