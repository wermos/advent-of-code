#include <fstream>
#include <iostream>
#include <string>

int main() {
    std::ifstream input{"inputs/day1.txt"};

    // we parse the input and to the computations required at the same time
    int running_sum = 0;
    int highest = 0;

    std::string line;

    while (std::getline(input, line)) {
        if (line == "") {
            if (running_sum > highest) {
                highest = running_sum;
            }
            // we reset the running sum to 0 for the next elf
            running_sum = 0;
        } else {
            int num = std::stoi(line);
            running_sum += num;
        }
    }

    // We need to process the last sum also.
    if (running_sum > highest) {
        highest = running_sum;
    }

    std::cout << highest << std::endl;
}