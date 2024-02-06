#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <string>

int calculate_value(char c) {
    // lowercase letters have a higher ASCII value than uppercase letters,
    // so we compare the incoming character with 'a'.
    //
    // The ASCII value of 'a' is 97, and the ASCII value of 'A' is 65. If the
    // incoming character compares to be greater than or equal to 'A', it
    // doesn't us anything because that is true for all letters.

    int value;

    if (static_cast<int>(c) >= static_cast<int>('a')) {
        // then we have a lowercase letter on our hands
        value = static_cast<int>(c) - static_cast<int>('a') + 1;
        // we add 1 to the result because 'a' has a priority value of 1.
    } else {
        // we have an uppercase letter on our hands
        value = static_cast<int>(c) - static_cast<int>('A') + 27;
        // we add 1 to the result because 'A' has a priority value of 27.
    }

    return value;
}

int main() {
    std::ifstream input{"inputs/day3.txt"};

    int total_priority = 0;

    std::string line;

    while (std::getline(input, line)) {
        std::string first = line.substr(0, line.length() / 2);
        std::string second = line.substr(line.length() / 2);

        std::set<char> compartment1{first.begin(), first.end()};
        std::set<char> compartment2{second.begin(), second.end()};
        std::set<char> common;

        std::set_intersection(compartment1.begin(), compartment1.end(), compartment2.begin(), compartment2.end(),
                              std::inserter(common, common.begin()));

        for (char c : common) {
            total_priority += calculate_value(c);
        }
    }

    std::cout << total_priority << '\n';
}