#include <fstream>
#include <iostream>
#include <string>
#include <unordered_set>

int main() {
    std::ifstream input{"inputs/day6.txt"};

    std::string line;

    std::getline(input, line);

    std::unordered_set<char> seen;

    for (int i = 0; i <= line.size() - 14; i++) {
        for (int j = i; j < i + 14; j++) {
            seen.insert(line[j]);
        }

        if (seen.size() == 14) {
            std::cout << i + 14 << '\n';
            break;
        } else {
            seen.clear();
        }
    }
}