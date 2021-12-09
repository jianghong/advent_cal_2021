use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut bitmap: Vec<Vec<u32>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let digits = parse_line(&line.unwrap());
        bitmap.push(digits);
    }

    let max_i = bitmap.len();
    let mut max_j = 0;
    let mut found_low_points: Vec<u32> = Vec::new();

    for (i, row) in bitmap.iter().enumerate() {
        max_j = row.len();
        for (j, digit) in row.iter().enumerate() {
            let mut lower_adjacent_digits: Vec<bool> = Vec::new();
            if i != 0 {
                // check above
                let above = i - 1;
                lower_adjacent_digits.push(digit < &bitmap[above][j]);
            }

            if i != max_i - 1 {
                // check below
                let below = i + 1;
                lower_adjacent_digits.push(digit < &bitmap[below][j]);
            }

            if j != 0 {
                // check left
                let left = j - 1;
                lower_adjacent_digits.push(digit < &bitmap[i][left]);
            }

            if j != max_j - 1 {
                // check right
                let right = j + 1;
                lower_adjacent_digits.push(digit < &bitmap[i][right]);
            }

            if lower_adjacent_digits.iter().all(|x| *x) {
                found_low_points.push(*digit);
            }
        }
    }
    println!("{:?}", found_low_points);
    // add 1 to each low point and sum
    let sum = found_low_points.iter().map(|x| x + 1).sum::<u32>();
    println!("part 1 {}", sum);
}

// 2199943210 -> [2, 2, 9, 9, 4, 3, 2, 1, 0]
fn parse_line(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();
    for c in line.chars() {
        digits.push(c.to_digit(10).unwrap());
    }
    digits
}