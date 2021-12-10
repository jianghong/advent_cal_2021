use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let filename = "src/input.txt";
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut forward = 0;
    let mut depth = 0;
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let split = l.split(" ").collect::<Vec<&str>>();
        let val = split[1].parse::<i32>().unwrap();
        if split[0] == "forward" {
            forward += val;
        } else if split[0] == "down" {
            depth += val;
        } else if split[0] == "up" {
            depth -= val;
        }
    }
    println!("Part 1 answer: {}", forward * depth);
}


fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let split = l.split(" ").collect::<Vec<&str>>();
        let val = split[1].parse::<i32>().unwrap();
        if split[0] == "forward" {
            forward += val;
            depth += aim * val;
        } else if split[0] == "down" {
            aim += val;
        } else if split[0] == "up" {
            aim -= val;
        }
    }
    println!("Part 2 answer: {}", forward * depth); // substract 1 to ignore the first comparison
}