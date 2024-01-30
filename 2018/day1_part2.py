with open("inputs/day1.txt", 'r') as f:
    total = 0
    list_of_totals_seen = [0]
    duplicate_found = False
    input_lines = f.readlines()
    # Preprocessing to make each run of the while loop faster:
    input_lines = [int(line.removesuffix("\n")) for line in input_lines]
    while not duplicate_found:
        for num in input_lines:
            total += num 
            if total in list_of_totals_seen:
                print(total)
                duplicate_found = True
                break
            else:
                list_of_totals_seen.append(total)
