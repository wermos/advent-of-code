#include <array>
#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>

class CPU {
   public:
    CPU() : current_cycle{0}, X{1}, values{}, add_instr{false}, add_val{0}, last_add_cycle{0} {}

    void addx(int val) {
        // just passes the value to be added to X into the CPU class. Also
        // sets the add_instr flag to `true`.
        add_val = val;
        add_instr = true;
        last_add_cycle = current_cycle;
    }

    void clock() {
        // increment the processor clock by one cycle, and take care of
        // whatever side effects need to happen.
        if (add_instr) {
            // process the add instruction as required
            current_cycle++;
            if (std::abs(current_cycle - last_add_cycle) == 2) {
                X += add_val;
                add_instr = false;
            }
        } else {
            // then it's a no op
            current_cycle++;
        }

        if (current_cycle % 40 == 20 && current_cycle <= 220) {
            values[current_cycle / 40] = X;
        }
    }

    int x() { return X; }

    int strength() {
        int strength = 0;

        for (int i = 0; i < 6; i++) {
            int cycle = 40 * i + 20;
            // std::cout << values[i] << '\n';
            strength += cycle * values[i];
        }

        return strength;
    }

   private:
    int current_cycle;
    int X;
    std::array<int, 6> values;  // records the value of X during the 20th,
    // 60th, 100th, 140th, 180th, and 220th cycles

    bool add_instr;      // is an add happening right now?
    int add_val;         // what is the value to be added in the register?
    int last_add_cycle;  // what is the cycle when the last add instruction
                         // was issued?
};

int main() {
    std::ifstream input{"inputs/day10-test2.txt"};

    CPU c;

    std::string line;

    while (std::getline(input, line)) {
        if (!(line == "noop")) {
            // it's an add

            // we abuse the fact that the first 5 characters in the string are "addx "
            int val = std::stoi(line.substr(5));
            c.addx(val);
        }
        c.clock();
        std::cout << c.x() << '\n';
    }
    c.clock();
    std::cout << c.x() << '\n';
    c.clock();
    std::cout << c.x() << '\n';

    // std::cout << c.strength() << std::endl;
}