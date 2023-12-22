def hash(string: str) -> int:
    current_value = 0
    
    for char in string:
        current_value += ord(char)
        current_value *= 17
        current_value %= 256
    
    return current_value

def read_input(path: str) -> list[str]:
    with open(path, "r") as f:
        # There is no newline in the inputs for this problem: Neither
        # in the test input nor the actual input.
        return f.readline().split(",")
    
if __name__ == "__main__":
    string_list = read_input("inputs/day15.txt")
    
    print(sum(map(hash, string_list)))