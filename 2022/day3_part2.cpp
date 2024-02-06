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

    std::string line1;
    std::string line2;
    std::string line3;

    while (std::getline(input, line1) && std::getline(input, line2) && std::getline(input, line3)) {
        std::set<char> rucksack1{line1.begin(), line1.end()};
        std::set<char> rucksack2{line2.begin(), line2.end()};
        std::set<char> rucksack3{line3.begin(), line3.end()};

        // `common1` stores the list of common items between `rucksack1` and
        // `rucksack2`
        std::set<char> common1;
        // `common2` stores the list of common items between all the rucksacks,
        // by comparing `common1` and `rucksack3`.
        std::set<char> common2;

        std::set_intersection(rucksack1.begin(), rucksack1.end(), rucksack2.begin(), rucksack2.end(),
                              std::inserter(common1, common1.begin()));

        std::set_intersection(common1.begin(), common1.end(), rucksack3.begin(), rucksack3.end(),
                              std::inserter(common2, common2.begin()));

        for (char c : common2) {
            total_priority += calculate_value(c);
        }
    }

    std::cout << total_priority << '\n';
}