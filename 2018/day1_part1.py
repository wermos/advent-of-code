with open("inputs/day1.txt", 'r') as f:
    total = 0
    for line in f.readlines():
        total += int(line.removesuffix("\n"))
    print(total)
