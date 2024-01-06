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

def calculate_ways_fast(ins: Instance) -> int:
    time = ins.time
    distance = ins.distance
    
    import ctypes
    # Loading the C++ DLL which does the number crunching
    handle = ctypes.cdll.LoadLibrary("./day6_part2_helper.so")
    
    # The relevant function in the .dll/.so takes in 2 uint64_t's
    answer = handle.calculate_ways(ctypes.c_uint64(time),
                                   ctypes.c_uint64(distance))
    
    # Python assumes that the return type is a C int by default, so
    # I don't need to do anything special with the return type.
    return answer

def calculate_ways_extension(ins: Instance) -> int:
    time = ins.time
    distance = ins.distance
    
    import sys
    sys.path.append("./modules/")
    import helper
    
    answer = helper.calculate_ways(time, distance)
    return answer

if __name__ == "__main__":
    instance = read_input("inputs/day6-part2.txt")

    # print(calculate_ways(instance))
    # print(calculate_ways_fast(instance))
    print(calculate_ways_extension(instance))