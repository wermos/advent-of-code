import re

total = 0

regex = re.compile(r"\d")

with open("day1.txt", "r") as f:
    for line in f.readlines():
        first = re.search(regex, line)[0]
        
        reversed_line = ''.join(list(reversed(line)))
        last = re.search(regex, reversed_line)[0]
        
        number = int(''.join([first, last]))
        
        total += number

print(total)