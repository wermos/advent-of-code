#include <fstream>
#include <iostream>
#include <cstdint>
#include <string>
#include <stack>

template <std::uint8_t N>
struct CrateStack {
    CrateStack() = default;

    std::array<std::stack<char>, N> crates;
};

template <std::uint8_t N>
void initialize(const std::ifstream& file, const CrateStack<N>& stack, const bool test = false) {
    if constexpr (test) {
        // then the first few 
    } else {
        
    }
}

int main() {
    std::ifstream input{"inputs/day5-test.txt"};
    
    std::string line;

    while (std::getline(input, line)) {
    
    }
}