import typing

Lens = typing.NewType("Lens", tuple[str, int])

def hash(string: str) -> int:
    current_value = 0

    for char in string:
        current_value += ord(char)
        current_value *= 17
        current_value %= 256

    return current_value

def process_hashmap(op_list: list[tuple]) -> int:
    boxes: list[list[Lens]] = [list() for _ in range(256)]
    
    for operation in op_list:
        if operation[-1] == "-":
            label, _ = operation
            box_num = hash(label)
            
            try:
                # We construct a list of the lens labels, and search for the
                # desired label. If it is found, we remove it from the list.
                label_list = [lens[0] for lens in boxes[box_num]]
                label_index = label_list.index(label)
                
                boxes[box_num].pop(label_index)
            except ValueError:
                # this means that the label was not found. In this case, we
                # want to do nothing, so we catch the exception and do
                # nothing with it. It's ugly but it works.
                pass
        else:
            label, focal_length = operation
            box_num = hash(label)

            try:
                # We construct a list of the lens labels, and search for the
                # desired label. If it is found, we modify it.
                label_list = [lens[0] for lens in boxes[box_num]]
                label_index = label_list.index(label)
                
                boxes[box_num][label_index] = Lens((label, int(focal_length)))
            except ValueError:
                # the label was not found in the list, so we simply add it in.
                boxes[box_num].append(Lens((label, int(focal_length))))
    
    power = 0
    for i in range(len(boxes)):
        box = boxes[i]
        
        focal_length = 0
        for j in range(len(box)):
            focal_length += (j + 1) * box[j][1]
        
        power += (i + 1) * focal_length
    
    return power

def read_input(path: str) -> list[tuple]:
    with open(path, "r") as f:
        # There is no newline in the inputs for this problem: Neither
        # in the test input nor the actual input.
        input_list = f.readline().split(",")

    operations: list[tuple] = list()
    for elem in input_list:
        if "-" in elem:
            operations.append((elem[:-1], elem[-1]))
        
        if "=" in elem:
            operations.append(tuple(elem.split("=")))

    return operations

if __name__ == "__main__":
    op_list = read_input("inputs/day15.txt")
    
    print(process_hashmap(op_list))