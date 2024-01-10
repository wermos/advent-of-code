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

So, since I'm doing all the [2015 problems](https://adventofcode.com/2015) in Java, `.ext` is `.java` in the `2015/` directory. On the other hand, since [2023](https://adventofcode.com/2023) is in Python, `.ext` for 2023 is `.py`.

#### Underscores as Filename Delimiters

The repo originally used hyphens as delimiters between the different parts of the program file names. I had to change the 2023 program file names to underscores, because hyphens were being interpreted as minus signs by the Python Interpreter, and this was preventing me from importing other source files as modules. For example, if I wanted to use the Part 1 scaffolding for Part 2, I was unable to do so.

For this reason, I changed all the filename delimiters to underscores over the [course](https://github.com/wermos/advent-of-code/commit/c2c1a665f577e1d8ec5516d3b67ac351c58ce41b) [of](https://github.com/wermos/advent-of-code/commit/152855e6d0cc3a2a39dea5f4431453a8ee8509e8) [four](https://github.com/wermos/advent-of-code/commit/874866a578a31422a96faf7d50c85723307e16b4) [commits](https://github.com/wermos/advent-of-code/commit/adbb58772390c8fe9b2bc42e9512da871a233db1).

[At the time of writing](https://github.com/wermos/advent-of-code/tree/869dcace4cc25d09ca49dcb1b9ab36a981b039d0), Part 1 code was used in the Part 2 solution for [Day 2](https://github.com/wermos/advent-of-code/blob/869dcace4cc25d09ca49dcb1b9ab36a981b039d0/2023/day2_part2.py#L1) and [Day 6](https://github.com/wermos/advent-of-code/blob/869dcace4cc25d09ca49dcb1b9ab36a981b039d0/2023/day6_part2.py#L5) of 2023.

Here is what the repo structure looks like, [at the time of writing](https://github.com/wermos/advent-of-code/tree/5be1a79b2c7ebe7b17b2a07c2df2fa7382ebac12):

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
│  ├─ src/
│  │  ├─ day6_part2_helper.hpp
│  │  ├─ day6_part2_helper_alt.cpp
│  │  ├─ day6_part2_helper.cpp
│  │ ⋮
│  ├─ modules/
│  │  ├─ helper.so
│  │  ├─ libday6_part2_helper.so
│  │ ⋮
│  ├─ CMakeLists.txt
│  ├─ __init__.py
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

### Python

TODO: Fill this section in.

### Special Cases

#### 2023 Day 6, Part 2

##### Motivation

For this problem, I first wrote a pure Python solution, which took around 4 seconds to run on my machine. This solution can be found in the [`calculate_ways`](https://github.com/wermos/advent-of-code/blob/adbb58772390c8fe9b2bc42e9512da871a233db1/2023/day6_part2.py#L13-L21) function.

4 seconds felt like an unacceptably long amount of time, especially because the problem boiled down to checking the interval for which a certain parabola is less than 0. This is a task that can be done with pen and paper relatively easily, though it would be boring and have messy calculations in this case.

My machine-friendly alternative was to create a range of numbers as a list (or array): $\{ 1, 2, \dots, t_{\text{max}} \}$. Applying an objective function (called $f$ here for simplicity) to each one to arrive at a new list/array $\{ f(1), f(2), \dots, f(t_{\text{max}}) \}$, and then counting how many of those elements are above a certain threshold $d$.



### Java

TODO: Fill this section in.

### C++

TODO: Fill this section in.
