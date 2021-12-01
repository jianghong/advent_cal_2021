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
    let mut count = 0;
    let mut previous = 0;
    for (index, line) in reader.lines().enumerate() {
        let val = line.unwrap().parse::<i32>().unwrap();
        if index != 0 {
            if val > previous {
                count += 1
            }
        }
        previous = val;

    }
    println!("Part 1 answer: {}", count);
}


fn part2(filename: &str) {
    let mut window_totals: Vec<i32> = Vec::new();
    let mut window_1 = 0;
    let mut window_2 = 0;
    let mut window_3 = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let val = line.unwrap().parse::<i32>().unwrap();
        window_1 += val;
        window_2 += val;
        window_3 += val;

        let m = index % 3;
        if m == 0 {
            if index != 0 {
                window_totals.push(window_1);
            }
            window_1 = 0
        } else if m == 1 {
            if index != 1 {
                window_totals.push(window_2);
            }
            window_2 = 0
        } else if m == 2 {
            window_totals.push(window_3);
            window_3 = 0
        }
    }

    let mut count = 0;
    let mut previous = 0;
    for val in window_totals {
        if val > previous {
            count += 1;
        }
        previous = val
    }
    println!("Part 2 answer: {}", count - 1); // substract 1 to ignore the first comparison
}