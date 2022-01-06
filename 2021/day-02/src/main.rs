use std::fs::File;
use std::io::{self, BufRead};

/*
Advent of Code 2021 Day 2
https://adventofcode.com/2021/day/2

Objective: Track position across two dimensions, then multiply those values together
*/

fn main() {
    let file = File::open("data/input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut vertical = 0i64;
    let mut horizontal = 0i64;
    let mut aim = 0i64;

    reader.lines().for_each(|input_line| {
        let line = input_line.unwrap();
        let mut iter = line.split_whitespace();

        let direction = iter.next().unwrap();
        let value = iter.next().unwrap().parse::<i64>().unwrap();

        match direction {
            "up" => aim -= value,
            "down" => aim += value,
            "forward" => {
                horizontal += value;
                vertical += aim * value;
            }
            _ => println!("unknown direction: {}", direction),
        }
    });

    println!("vertical: {}", vertical);
    println!("horizontal: {}", horizontal);
    println!("multiplied: {}", vertical * horizontal);
}
