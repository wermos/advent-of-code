total = 0

mapping = {
    "zero"  : "0",
    "one"   : "1",
    "two"   : "2",
    "three" : "3",
    "four"  : "4",
    "five"  : "5",
    "six"   : "6",
    "seven" : "7",
    "eight" : "8",
    "nine"  : "9"
}

def calculate_number(line: str) -> int:
    min_indices = list()
    max_indices = list()
    
    # Since these are all single digit numbers, we can easily process this
    for i in range(0, 10):
        num_string = str(i)
        min_indices.append(line.find(num_string))
        max_indices.append(line.rfind(num_string))
    
    min_idx = 0 # By default, we set the minimum index to 0.
    if not all(n == -1 for n in min_indices):
        # if all the indices are not -1, then there was one digit which was
        # found somewhere in the string. We remove all the -1 entries and
        # then find the minimum of the list.
        min_idx = min(filter(lambda n: n != -1, min_indices))
    max_idx = max(max_indices)
    
    first_digit = line[min_idx]
    last_digit = line[max_idx]
    
    for num in mapping.keys():
        idx = line.find(num)
        
        if idx != -1 and idx <= min_idx:
            # We use the condition `idx <= min_idx` for a very specific reason.
            # It is not possible for `idx` to be equal to `min_idx`, because
            # would imply that at `idx` is a character which is both a digit
            # as well as an alphabetical letter.
            #
            # The equality condition is useful for when no digit was found. In
            # that case, `min_idx` is already 0, and it is possible that there
            # was a spelled out number at index 0, in "zero9two" for example.
            #
            # The equality condition accounts for such a possibility as well.
            first_digit = mapping[num]
            min_idx = idx
        
        idx = line.rfind(num)
        
        if idx != -1 and idx > max_idx:
            last_digit = mapping[num]
            max_idx = idx
    
    return int(first_digit + last_digit)

with open("inputs/day1.txt", "r") as f:
    for line in f.readlines():
        total += calculate_number(line)

print(total)