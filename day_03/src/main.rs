use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut gamma: Vec<&str> = Vec::new();
    let mut epsilon: Vec<&str> = Vec::new();
    let mut counts_0: Vec<i32> = Vec::new();
    let mut counts_1: Vec<i32> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();

        for (i, c) in l.chars().enumerate() {
            if c == '0' {
                if i >= counts_0.len() {
                    counts_0.push(1)
                } else {
                    counts_0[i] += 1
                }
            } else {

                if i >= counts_1.len() {
                    counts_1.push(1)
                } else {
                    counts_1[i] += 1
                }
            }
        }
    }

    for i in 0..counts_0.len() {
        if counts_0[i] > counts_1[i] {
            gamma.push("0");
            epsilon.push("1");
        } else {
            gamma.push("1");
            epsilon.push("0");
        }
    }

    let gamma_int = isize::from_str_radix(&gamma.join(""), 2).unwrap();
    let e_int = isize::from_str_radix(&epsilon.join(""), 2).unwrap();
    println!("Part 1 answer: {}", gamma_int * e_int);
}


fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        lines.push(l);
    }
    let mut l = lines.len();
    
    // get lines
    // determine counts
    // add to result winner
    // filter out lines
    // end of lines == len 1
    let mut counts_0: Vec<i32> = Vec::new();
    let mut counts_1: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut result: Vec<&str> = Vec::new();

    while l != 1 {
        println!{"i: {}", i}
        println!{"len: {}", lines.len()}
        for line in &lines {
            let c = line.chars().nth(i).unwrap();
            if c == '0' {
                if i >= counts_0.len() {
                    counts_0.push(1)
                } else {
                    counts_0[i] += 1
                }
            } else {
                if i >= counts_1.len() {
                    counts_1.push(1)
                } else {
                    counts_1[i] += 1
                }
            }
        }
        if counts_0[i] > counts_1[i] {
            result.push("0")
        } else {
            result.push("1")
        }

        lines.retain(|l| l.starts_with(&result.join("")));
        l = lines.len();
        i += 1
    }
    let ox = &lines[0];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        lines.push(l);
    }
    let mut l = lines.len();
    
    // get lines
    // determine counts
    // add to result winner
    // filter out lines
    // end of lines == len 1
    let mut counts_0: Vec<i32> = Vec::new();
    let mut counts_1: Vec<i32> = Vec::new();
    let mut i = 0;
    let mut result: Vec<&str> = Vec::new();

    while l != 1 {
        println!{"i: {}", i}
        println!{"len: {}", lines.len()}
        for line in &lines {
            let c = line.chars().nth(i).unwrap();
            if c == '0' {
                if i >= counts_0.len() {
                    counts_0.push(1)
                } else {
                    counts_0[i] += 1
                }
            } else {
                if i >= counts_1.len() {
                    counts_1.push(1)
                } else {
                    counts_1[i] += 1
                }
            }
        }
        if counts_0[i] > counts_1[i] {
            result.push("1")
        } else {
            result.push("0")
        }

        lines.retain(|l| l.starts_with(&result.join("")));
        l = lines.len();
        i += 1
    }
    let co = &lines[0];

    println!("ox: {}, co: {}", ox, co);
    let ox_int = isize::from_str_radix(ox, 2).unwrap();
    let co_int = isize::from_str_radix(co, 2).unwrap();
    println!("Part 2 answer: {}", ox_int * co_int);
}