# Advent of Code

This repo stores all the Advent of Code solutions that I have written.

## Goals

In its final form, I want the repo to store the complete solution to every Advent of Code problem.

TODO: Add more info.

### Progress

In order to be able to easily see the progress that I have made so far, I added a few banners below, which show the progress that has been made according to a couple of different metrics.

#### Overall

![Progress Bar to show how much progress has been made](https://progress-bar.dev/4/?title=Progress)

#### By Year

![Progress Bar to show how much progress has been made in the 2015 problems](https://progress-bar.dev/4/?title=2015)
![Progress Bar to show how much progress has been made in the 2016 problems](https://progress-bar.dev/0/?title=2016)
![Progress Bar to show how much progress has been made in the 2017 problems](https://progress-bar.dev/0/?title=2017)
![Progress Bar to show how much progress has been made in the 2018 problems](https://progress-bar.dev/4/?title=2018)
![Progress Bar to show how much progress has been made in the 2019 problems](https://progress-bar.dev/0/?title=2019)
![Progress Bar to show how much progress has been made in the 2020 problems](https://progress-bar.dev/0/?title=2020)
![Progress Bar to show how much progress has been made in the 2021 problems](https://progress-bar.dev/0/?title=2021)
![Progress Bar to show how much progress has been made in the 2022 problems](https://progress-bar.dev/8/?title=2022)
![Progress Bar to show how much progress has been made in the 2023 problems](https://progress-bar.dev/22/?title=2023)

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

[At the time of writing (this subsection)](https://github.com/wermos/advent-of-code/tree/869dcace4cc25d09ca49dcb1b9ab36a981b039d0), Part 1 code was used in the Part 2 solution for [Day 2](https://github.com/wermos/advent-of-code/blob/869dcace4cc25d09ca49dcb1b9ab36a981b039d0/2023/day2_part2.py#L1) and [Day 6](https://github.com/wermos/advent-of-code/blob/869dcace4cc25d09ca49dcb1b9ab36a981b039d0/2023/day6_part2.py#L5) of 2023.

Here is what the repo structure looks like, [at the time of writing (this subsection)](https://github.com/wermos/advent-of-code/tree/5be1a79b2c7ebe7b17b2a07c2df2fa7382ebac12):

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

Since there are multiple languages used in the repository, it makes sense to talk about the version(s) with which the code has been tested, and the version(s) with which the code was run.

### Python

Most of the Python solutions should work even with an older Python version like 3.9. That being said, the only guarantee I can give is for Python 3.11 onwards, since that is the version I used to run my Python code.

### Java

[At the time of writing this subsection](https://github.com/wermos/advent-of-code/tree/c0c594b6020cb0262f2964596482c926b130938b), the only Java code in the repository is the Day 1 Parts 1 and 2 solution for 2015. I ran the code using [Java 22 Early Access](https://jdk.java.net/22/). That being said, it _should_ work on any Java version 8 onwards.

### C++

For best results, the code should be compiled for C++20 or later. All the C++ code currently in the repository works with C++20 and above, though at some point I might end up using a C++23-only construct, at which point I would have to upgrade the C++ version requirement accordingly.

## Running The Programs

One important thing to note is that the working directory is assumed to be the directory in which the program file resides. So, if we want to run 2023's Day 4 Part 2 solution, we would run `python day4_part2.py` from inside the `2023/` directory.

Apart from that, the only other important things to note is that there is a CMake script to help build the C++ libraries required for the alternative 2023 Day 6 Part 2 solutions. To use those, run

```bash
cmake -S. -Bmodules
```

with the `2023/` as your current working directory, and then run

```bash
cmake --build modules
```

After that, you should be able to run `python day6_part2.py` without any issues.

## Special Cases

This section deals with certain special cases where more work was done than one would expect, or where things were done differently than one would expect. In each case, I discuss why it was required, and what exactly I did.

### 2015 Day 1

Originally, the 2015 Day 1 Part 1 solution was the thing which started the whole repository. However, when I was going back and checking old years on the Advent of Code website, I realized that my 2015 Day 1 was not finished. I realized then that I must have used a different account to do it.

Since I already had the code, I decided to run it on the input file that was already in the repo, which didn't work because the input is different for each user. Once I redownloaded the input, and reran the program, I got the required answer.

After reading Part 2 of Day 1, I decided it was so simple that I might as well finish it, which is why I added `2015/day1_part2.java`.

For this reason, even though the Day 1 Part 1 file was originally added in [41420bd](https://github.com/wermos/advent-of-code/tree/41420bd4e71bef81f30d2f98c6270ee1f27f28fd), it had to be redone in [f2be2fc](https://github.com/wermos/advent-of-code/commit/f2be2fc14d133e6cda5b5aebe336a6f984290fb6).

### 2018 Day 1

A similar case as the one above. I redownloaded the input files and reran the programs.

### 2023 Day 6, Part 2

#### Motivation

For this problem, I first wrote a pure Python solution, which took around 4 seconds to run on my machine. This solution can be found in the [`calculate_ways`](https://github.com/wermos/advent-of-code/blob/adbb58772390c8fe9b2bc42e9512da871a233db1/2023/day6_part2.py#L13-L21) function.

4 seconds felt like an unacceptably long amount of time, especially because the problem boiled down to finding the interval for which a certain parabola is less than 0. This is a task that can be done with pen and paper relatively easily, though it would be boring and have messy calculations in this case.

#### The Algorithm

My machine-friendly alternative was to create a range of numbers as a list (or array): $`\{ 1, 2, \dots, t_{\text{max}} \}`$. Applying an objective function (called $f$ here for simplicity) to each one to arrive at a new list/array $`\{ f(1), f(2), \dots, f(t_{\text{max}}) \}`$, and then counting how many of those elements are above a certain threshold $d$.
