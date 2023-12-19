from day6_part1 import Instance

def read_input(path: str) -> Instance:
    with open(path, "r") as f:
        # The first element in the first line is the word "Time:"
        time = f.readline().rstrip("\n").split()[1]

        # The first element in the second line is the word "Distance:"
        distance = f.readline().rstrip("\n").split()[1]

        return Instance(int(time), int(distance))

def calculate_ways(ins: Instance) -> int:
    time = ins.time
    distance = ins.distance
    
    f = lambda t: t * (time - t)
    
    distances = [f(t) for t in range(time)]
    
    return len(list(filter(lambda d: d > distance, distances)))

if __name__ == "__main__":
    instance = read_input("inputs/day6-part2.txt")

    print(calculate_ways(instance))