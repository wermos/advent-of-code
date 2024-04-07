from collections import namedtuple

Part = namedtuple("Part", "x, m, a, s")

# Each rule has a name, a condition, and a result.
#
# The name is straightforward: It's just a string.
#
# The condition has 3 parts: a variable name for checking, the operator (either
# greater than or less than), and value we are comparing it to.
#
# The result is either a rule name or "R" or "A". I will treat "R" as `False`
# and "A" as `True` for simplicity.
#
# With all this in mind, we create a `Rule` class that stores all this
# information.
class Rule:
    def __init__(self, name: str, var_name: str, greater_than: bool, comparison_value: int, result: str) -> None:
        self.name = name
        
        # We just read the data in verbatim and store it.
        self.var_name = var_name
        self.greater_than = greater_than
        self.comparison_value = comparison_value
        
        # The `Rule` class will be a faithful reconstruction of the input
        # data, so I won't convert "R" and "A" False and True yet.
        self.result = result
    
    def check(self, part: Part) -> bool:
        if self.greater_than:
            if self.var_name == "x":
                return part.x > self.comparison_value
            elif self.var_name == "m":
                return part.m > self.comparison_value
            elif self.var_name == "a":
                return part.a > self.comparison_value
            else:
                return part.s > self.comparison_value
        else:
            if self.var_name == "x":
                return part.x < self.comparison_value
            elif self.var_name == "m":
                return part.m < self.comparison_value
            elif self.var_name == "a":
                return part.a < self.comparison_value
            else:
                return part.s < self.comparison_value

# As per the problem statement, a workflow consists of multiple rules bundled
# together.
class Workflow:
    def __init__(self) -> None:
        self.rule_list = list()
    
    def process(self, part: Part):
        for rule in self.rule_list:
            result = rule.check(part)
            
            if result == "R":
                return False
            elif result == "A":
                return True
            else:
                return result


def read_input(path: str) -> tuple[list[str], list[Part]]:
    with open(path, "r") as f:
        reading_parts = False
        
        workflow_list: list[str] = list()
        parts_list: list[Part] = list()
        for line in f:
            if line == '\n':
                reading_parts = True
                continue
            
            if reading_parts:
                # We are reading in the parts' information
                
                # removing the curly braces and splitting on commas
                values = line[1:-2].split(',')
                
                # We assume that the order in which the 4 values are given is
                # the same for all the parts. I.e., we receive `x` first, then
                # `m`, then `a`, and finally `s`.
                #
                # Furthermore, we assume that the format is `_=num` for all the
                # numbers, where `_` is `x`, `m`, `a`, or `s`, and `num` is the
                # number.
                x = values[0][2:]
                m = values[1][2:]
                a = values[2][2:]
                s = values[3][2:]
                parts_list.append(Part(x, m, a, s))
            else:
                # We are reading in the rules list
                pass
    
    return workflow_list, parts_list

if __name__ == "__main__":
    workflow_list, parts_list = read_input("inputs/day19-test.txt")