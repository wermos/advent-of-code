from __future__ import annotations
import math

class Instance:
    def __init__(self, time: int, distance: int) -> None:
        self.time = time
        self.distance = distance
    
    @classmethod
    def from_tuple(cls, data: tuple[str, str]):
        return cls(int(data[0]), int(data[1]))

def read_input(path: str) -> list[Instance]:
    with open(path, "r") as f:
        # The first element in the first line is the word "Time:"
        times = f.readline().rstrip("\n").split()[1:]
        
        # The first element in the second line is the word "Distance:"
        distances = f.readline().rstrip("\n").split()[1:]
        
        return list(map(Instance.from_tuple, zip(times, distances)))

def calculate_ways(ins: Instance) -> int:
    time = ins.time
    distance = ins.distance
    
    counter = 0
    
    for t in range(time):
        v = t
        distance_traveled = v * (time - t)
        
        if distance_traveled > distance:
            counter += 1
    
    return counter

if __name__ == "__main__":
    instance_list = read_input("inputs/day6.txt")
    
    print(math.prod(list(map(calculate_ways, instance_list))))