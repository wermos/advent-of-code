# Advent of Code

This repo stores all the Advent of Code solutions that I have written.

## Goals

In its final form, I want the repo to store the complete solution to every Advent of Code problem.

TODO: Add more info.

## Repository Structure

The repository structure is quite simple. Assuming that `advent-of-code` is the top-level directory, it has a directory for each year.

### Inputs

Inside each directory, there is an `inputs/` directory which stores the input data for each day in `.txt` format. The naming scheme that was followed was `dayX.txt`, where `X` is the day number. For example, the input for the 16<sup>th</sup> day is `day16.txt`.

In many cases, I also added the example input they gave in the problem description to the repo in order to use it as a test run for my programs, to check for bugs. The naming scheme used for these files are simple:

- If there was only 1 test input file required, then the naming scheme is `dayX-test.txt`,
- and if a separate test input file was required for each part, then the naming scheme is `dayX-partY-test.txt`,

where `X` is the day number like before, and `Y` is either `1` or `2`, depending on which part the test input is for.

### Solutions

The format for the solutions is very straightforward: Each program is named `dayX_partY.ext`, where, like before, `X` is the day number, `Y` is the part number (so either `1` or `2`), and `.ext` is the usual source file extension used for that program.

So, since I'm doing all the [2015 problems](https://adventofcode.com/2015) in Java, `.ext` is `.java`. On the other hand, since [2023](https://adventofcode.com/2023) is in Python, `.ext` for 2023 is `.py`.

The repo originally used hyphens as delimiters between the different parts of the program file names. I had to change the 2023 program file names to underscores, because hyphens were being interpreted as minus signs by the Python Interpreter, and this was preventing me from importing 

Here is what the repo structure looks like, [at the time of writing](https://github.com/wermos/advent-of-code/tree/adbb58772390c8fe9b2bc42e9512da871a233db1):

```html
advent-of-code/
├─ 2015/
│  ├─ inputs/
│  │  ├─ day1.txt
│  │ ⋮
│  ├─ day1_part1.java
│  ├─ day1_part2.java
  ⋮
├─ 2018/
│  ├─ inputs/
│  │  ├─ day1.txt
│  │ ⋮
│  ├─ day1_part1.java
│  ├─ day1_part2.java
  ⋮
├─ 2023/
│  ├─ inputs/
│  │  ├─ day1.txt
│  │ ⋮
│  ├─ day1_part1.py
│  ├─ day1_part2.py
  ⋮
│  ├─ day6_part1.py
│  ├─ day6_part2.py
  ⋮
```

There are many holes, and entire years are currently missing, but those will (hopefully) be filled in over time 🤞

## Language Requirements

TODO: Write some introductory text.

### Java

TODO: Fill this section in.

### C++

TODO: Fill this section in.

### Python

TODO: Fill this section in.

### Special Cases

#### 2023 Day 6, Part 2

For this problem, I first wrote a pure Python solution, which took around 4 seconds to run on my machine. This solution can be found in the [`calculate_ways`](https://github.com/wermos/advent-of-code/blob/adbb58772390c8fe9b2bc42e9512da871a233db1/2023/day6_part2.py#L13-L21) function.

TODO: Finish filling this section in.
