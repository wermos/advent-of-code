#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>

int main() {
    std::ifstream input{"inputs/day6.txt"};

    std::string line;

    std::getline(input, line);

    std::unordered_set<char> seen;

    for (int i = 0; i <= line.size() - 4; i++) {
        // std::cout << "i = " << i << '\n';
        for (int j = i; j < i + 4; j++) {
            // std::cout << "\tj = " << j << '\n';
            seen.insert(line[j]);
        }

        // std::cout << seen.size() << '\n';

        if (seen.size() == 4) {
            std::cout << i + 4 << '\n';
            break;
        } else {
            seen.clear();
        }
    }
}