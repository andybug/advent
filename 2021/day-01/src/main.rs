use std::fs::File;
use std::io::{self, BufRead};

/*
Advent of Code 2021 Day 1
https://adventofcode.com/2021/day/1

Objective: count the number of increases in a file between the previous measurement and the current
*/

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut prev = 0u64;
    let mut increases = 0u64;

    reader.lines().enumerate().for_each(|(i, line)| {
        let curr = line.unwrap().parse::<u64>().unwrap();

        if i > 0 && curr > prev {
            increases += 1;
        }

        prev = curr;
    });

    println!("increases: {}", increases);
}

/* correct output is 1711 */
