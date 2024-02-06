#include <fstream>
#include <iostream>
#include <string>
#include <utility>

bool process(std::pair<int, int> p1, std::pair<int, int> p2) {
    // Decomposing the two pairs into [a_i, b_i] intervals.
    auto [a_1, b_1] = p1;
    auto [a_2, b_2] = p2;

    if (b_1 < a_2 || b_2 < a_1) {
        return false;
    } else {
        return true;
    }
}

int main() {
    std::ifstream input{"inputs/day4.txt"};

    int count = 0;
    std::string line;

    while (std::getline(input, line)) {
        auto comma_idx = line.find(',');

        auto first_range = line.substr(0, comma_idx);
        auto second_range = line.substr(comma_idx + 1);

        auto hypen_idx = first_range.find('-');
        std::pair<int, int> p1{std::stoi(first_range.substr(0, hypen_idx)),
                               std::stoi(first_range.substr(hypen_idx + 1))};

        hypen_idx = second_range.find('-');
        std::pair<int, int> p2{std::stoi(second_range.substr(0, hypen_idx)),
                               std::stoi(second_range.substr(hypen_idx + 1))};

        if (process(p1, p2)) {
            count++;
        }
    }

    std::cout << count << '\n';
}