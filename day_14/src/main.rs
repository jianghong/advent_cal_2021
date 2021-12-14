use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part2() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());

    let mut template = String::new();
    let mut patterns: HashMap<String, String> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            template = line.clone();
            continue;
        } else if line == "" {
            continue;
        }
        let split = line.split(" -> ").collect::<Vec<&str>>();
        patterns.insert(split[0].to_string(), split[1].to_string());
    }
    // NNCB
    // { CH: B, HH: N .. etc}

    let mut pattern_count: HashMap<String, u64> = HashMap::new();
    // iterate over sliding window of template
    for i in 0..template.len() - 1  {
        let mut pattern = template.chars().nth(i).unwrap().to_string();
        pattern.push(template.chars().nth(i+1).unwrap());
        let count = pattern_count.entry(pattern).or_insert(0);
        *count += 1;
    }
    let steps = 40;
    let mut char_count: HashMap<char, u64> = HashMap::new();
    char_count.insert(template.chars().nth(0).unwrap(), 1);
    for s in 0..steps {
        let mut new_pattern_count: HashMap<String, u64> = HashMap::new();
        for (pattern, count) in pattern_count.clone() {
            let mut new_pattern = String::new();
            let char_to_insert: String = patterns.get(&pattern).unwrap().clone();
            new_pattern.push(pattern.chars().nth(0).unwrap());
            new_pattern.push(char_to_insert.chars().nth(0).unwrap());
            new_pattern.push(pattern.chars().nth(1).unwrap());

            let first_slice = new_pattern_count.entry(new_pattern[0..2].to_string()).or_insert(0);
            *first_slice += count;

            let second_slice = new_pattern_count.entry(new_pattern[1..].to_string()).or_insert(0);
            *second_slice += count;

            if s == steps - 1 {
                let new_pattern_chars = new_pattern.chars().collect::<Vec<char>>();
                let pattern_char_count = char_count.entry(new_pattern_chars[1]).or_insert(0);
                *pattern_char_count += count;
                let pattern_char_count = char_count.entry(new_pattern_chars[2]).or_insert(0);
                *pattern_char_count += count;                              
            }
        }
        pattern_count = new_pattern_count;
        // println!("step s {}: {}", s, result);
    }
    // println!("{:?}", pattern_count);
    // println!("{}", result);

    // find char with max count in char_count
    let mut max_char: char = ' ';
    let mut max_count: u64 = 0;
    let mut min_char: char = ' ';
    let mut min_count: u64 = 3621232527571;
    for (c, count) in char_count {
        if count > max_count {
            max_char = c;
            max_count = count;
        }
        if count < min_count {
            min_char = c;
            min_count = count;
        }
    }
    println!("max char {} {}", max_char, max_count);
    println!("min char {} {}", min_char, min_count);
    println!("part 2 {}", max_count - min_count);
}


fn part1() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());

    let mut template = String::new();
    let mut patterns: HashMap<String, String> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 0 {
            template = line.clone();
            continue;
        } else if line == "" {
            continue;
        }
        let split = line.split(" -> ").collect::<Vec<&str>>();
        patterns.insert(split[0].to_string(), split[1].to_string());
    }
    // NNCB
    // { CH: B, HH: N .. etc}

    let mut pattern_count: HashMap<String, u64> = HashMap::new();
    // iterate over sliding window of template
    for i in 0..template.len() - 1  {
        let mut pattern = template.chars().nth(i).unwrap().to_string();
        pattern.push(template.chars().nth(i+1).unwrap());
        let count = pattern_count.entry(pattern).or_insert(0);
        *count += 1;
    }
    let steps = 10;
    let mut result = template[0..1].to_string();
    for s in 0..steps {
        let mut new_pattern_count: HashMap<String, u64> = HashMap::new();
        for (pattern, count) in pattern_count.clone() {
            let mut new_pattern = String::new();
            let char_to_insert: String = patterns.get(&pattern).unwrap().clone();
            new_pattern.push(pattern.chars().nth(0).unwrap());
            new_pattern.push(char_to_insert.chars().nth(0).unwrap());
            new_pattern.push(pattern.chars().nth(1).unwrap());

            let first_slice = new_pattern_count.entry(new_pattern[0..2].to_string()).or_insert(0);
            *first_slice += count;

            let second_slice = new_pattern_count.entry(new_pattern[1..].to_string()).or_insert(0);
            *second_slice += count;

            if s == steps - 1 {
                for _ in 0..count {
                    result.push_str(&new_pattern[1..3])
                }
            }
        }
        pattern_count = new_pattern_count;
        // println!("step s {}: {}", s, result);
    }
    // println!("{:?}", pattern_count);
    // println!("{}", result);
    println!("result len {}", result.len());
    let mut char_count: HashMap<char, u64> = HashMap::new();
    for c in result.chars() {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    // find char with max count in char_count
    let mut max_char: char = ' ';
    let mut max_count: u64 = 0;
    let mut min_char: char = ' ';
    let mut min_count: u64 = 99999;
    for (c, count) in char_count {
        if count > max_count {
            max_char = c;
            max_count = count;
        }
        if count < min_count {
            min_char = c;
            min_count = count;
        }
    }
    println!("max char {} {}", max_char, max_count);
    println!("min char {} {}", min_char, min_count);
    println!("part 1 {}", max_count - min_count);
}
