import timeit

import numpy as np

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
    handle = ctypes.cdll.LoadLibrary("./modules/libday6_part2_helper.so")
    
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

def calculate_ways_numpy(ins: Instance) -> int:
    time = ins.time
    distance = ins.distance
    
    time_range = np.arange(1, time, 1)
    
    def f(t):
        return t * (time - t)
    
    distances = f(time_range)
    
    return np.count_nonzero(distances > distance)

if __name__ == "__main__":
    instance = read_input("inputs/day6-part2.txt")

    # print(calculate_ways(instance))
    # print(calculate_ways_fast(instance))
    # print(calculate_ways_extension(instance))
    # print(calculate_ways_numpy(instance))
    
    print("Pure Python implementation:")
    t1 = timeit.Timer("calculate_ways(Instance(47707566, 282107911471062))", setup="from day6_part2 import Instance, calculate_ways")
    num_loops, time = t1.autorange()
    print(f"{time / num_loops} seconds per loop\n")
    
    print("C++ `ctypes` implementation:")
    t2 = timeit.Timer("calculate_ways_fast(Instance(47707566, 282107911471062))", setup="from day6_part2 import Instance, calculate_ways_fast")
    num_loops, time = t2.autorange()
    print(f"{time / num_loops} seconds per loop\n")
    
    print("C++ extension-based implementation:")
    t3 = timeit.Timer("calculate_ways_extension(Instance(47707566, 282107911471062))", setup="from day6_part2 import Instance, calculate_ways_extension")
    num_loops, time = t3.autorange()
    print(f"{time / num_loops} seconds per loop\n")
    
    print("NumPy implementation:")
    t4 = timeit.Timer("calculate_ways_numpy(Instance(47707566, 282107911471062))", setup="from day6_part2 import Instance, calculate_ways_numpy")
    num_loops, time = t4.autorange()
    print(f"{time / num_loops} seconds per loop\n")