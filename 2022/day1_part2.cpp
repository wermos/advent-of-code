#include <fstream>
#include <iostream>
#include <string>

struct Top3 {
    int a;
    int b;
    int c;

    Top3() : a{0}, b{0}, c{0} {}

    void process(int num) {
        if (num > a) {
            c = b;
            b = a;
            a = num;
        } else if (num > b) {
            c = b;
            b = num;
        } else if (num > c) {
            c = num;
        }
    }

    int sum() { return a + b + c; }
};

int main() {
    std::ifstream input{"inputs/day1.txt"};

    // we parse the input and to the computations required at the same time
    Top3 t;
    int running_sum = 0;

    std::string line;

    while (std::getline(input, line)) {
        if (line == "") {
            t.process(running_sum);

            // we reset the running sum to 0 for the next elf
            running_sum = 0;
        } else {
            int num = std::stoi(line);
            running_sum += num;
        }
    }

    // We must process the last running sum also.
    t.process(running_sum);

    std::cout << t.sum() << std::endl;
}