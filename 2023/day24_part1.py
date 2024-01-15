from itertools import combinations
import typing

Position = typing.NewType("Position", tuple[int, int, int])
Velocity = typing.NewType("Velocity", tuple[int, int, int])

class Hailstone():
    def __init__(self, pos: Position, vel: Velocity) -> None:
        self.position = pos
        self.velocity = vel

    def __repr__(self) -> str:
        return f"Position: ({self.position[0]}, {self.position[1]}, {self.position[2]})\n" \
               f"Velocity: ({self.velocity[0]}, {self.velocity[1]}, {self.velocity[2]})"

def read_input(path: str) -> list[Hailstone]:
    hail_list: list[Hailstone] = list()

    with open(path, "r") as f:
        for line in f:
            pos_str, vel_str = line.split("@")

            x, y, z = pos_str.strip().split(",")
            v_x, v_y, v_z = vel_str.strip().split(",")

            hail_list.append(Hailstone(
                Position((int(x), int(y), int(z))),
                Velocity((int(v_x), int(v_y), int(v_z)))
            ))

    return hail_list

def process(hailstone1: Hailstone, hailstone2: Hailstone,
            lower_lim: int, upper_lim: int):
    # We ignore the Z axis, as the problem description states.
    p1_x, p1_y, _ = hailstone1.position
    v1_x, v1_y, _ = hailstone1.velocity

    p2_x, p2_y, _ = hailstone2.position
    v2_x, v2_y, _ = hailstone2.velocity
    
    # If we let p1, v1 and p2, v2 denote the position and the velocity of the
    # first and second hailstone respectively, then we can represent p1 and p2
    # as a function of times t1 and t2 as follows:
    #
    # p1_new = p1_initial + v1 * t1
    # p2_new = p2_initial + v2 * t2
    #
    # Note that p1_new, p2_new, p1_initial, p2_initial, v1, and v2 are all
    # vectors in the above equations.
    #
    # We want p1_new = p2_new, so after rearranging, we get the following
    # equation:
    #
    # p1_initial - p2_initial = v2 * t2 - v1 * t1
    #
    # After breaking down the above equation into its x and y components (we
    # ignore z as the part 1 problem statement states), we get the following
    # system of linear equations:
    #
    # p1_initial_x - p2_initial_x = v2_x * t2 - v1_x * t1         (x-direction)
    # p1_initial_y - p2_initial_y = v2_y * t2 - v1_y * t1         (y-direction)
    #
    # We can apply Cramer's Rule to this system. If the determinant in the
    # denominator of the solutions is 0, then the two hailstones never
    # intersect. That's what we check for below:
    denom = -v2_x * v1_y + v1_x * v2_y

    if denom != 0:
        # Then we calculate the actual intersection point and see if it's
        # within the required range
        d_x = p1_x - p2_x
        d_y = p1_y - p2_y

        # Using Cramer's Rule on the simultaneous equations
        # We need to calculate both t1 and t2 to ensure that both are
        # nonnegative.
        t2 = (-d_x * v1_y + d_y * v1_x) / denom
        t1 = (d_y * v2_x - d_x * v2_y) / denom

        if t1 >= 0 and t2 >= 0:
            # Only one of the times is required to calculate the intersection
            # point. In this case, we use t2.
            new_x = p2_x + v2_x * t2
            new_y = p2_y + v2_y * t2

            if lower_lim <= new_x <= upper_lim and lower_lim <= new_y <= upper_lim:
                return True
            else:
                return False
        else:
            return False
    else:
        return False

if __name__ == "__main__":
    hail_list = read_input("inputs/day24.txt")

    lower_lim = 200_000_000_000_000
    upper_lim = 400_000_000_000_000

    counter = 0
    for hailstone1, hailstone2 in combinations(hail_list, 2):
        # We use itertools.combinations to prevent double-counting.
        result = process(hailstone1, hailstone2, lower_lim, upper_lim)
        if result:
            counter += 1

    print(counter)