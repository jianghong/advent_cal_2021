use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part2() {
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
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut basin_sizes: Vec<u32> = Vec::new();

    for (i, row) in bitmap.iter().enumerate() {
        max_j = row.len();
        for (j, digit) in row.iter().enumerate() {
            // continue if (i, j) is already visited
            if visited.contains(&(i, j)) {
                continue;
            }

            // if digit is 9, visit and continue
            if *digit == 9 {
                visited.push((i, j));
                continue;
            }
            // call basin_size
            let basin_size = basin_size(bitmap.clone(), &mut visited, i, j, max_i, max_j);
            basin_sizes.push(basin_size);
        }
    }

    // get top 3 basin sizes
    let mut top_3_basin_sizes: Vec<u32> = basin_sizes.clone();
    top_3_basin_sizes.sort();
    top_3_basin_sizes.reverse();
    top_3_basin_sizes.truncate(3);
    // multiply top_3_basin_sizes
    let mut product = 1;
    for basin_size in top_3_basin_sizes {
        product *= basin_size;
    }
    println!("part2 {}", product);
}

fn basin_size(bitmap: Vec<Vec<u32>>, visited: &mut Vec<(usize, usize)>, i: usize, j: usize, max_i: usize, max_j: usize) -> u32 {
    let digit = bitmap[i][j];
    let mut total_size = 0;
    if digit == 9 {
        visited.push((i, j));
        return 0;
    } else if visited.contains(&(i, j)) {
        return 0;
    } else {
        total_size += 1;
        visited.push((i, j));
        // visit top
        if i != 0 {
            total_size += basin_size(bitmap.clone(), visited, i - 1, j, max_i, max_j);
        }
        // visit bottom
        if i != max_i - 1 {
            total_size += basin_size(bitmap.clone(), visited, i + 1, j, max_i, max_j);
        }
        // visit left
        if j != 0 {
            total_size += basin_size(bitmap.clone(), visited, i, j - 1, max_i, max_j);
        }
        // visit right
        if j != max_j - 1 {
            total_size += basin_size(bitmap.clone(), visited, i, j + 1, max_i, max_j);
        }
    }
    return total_size
}

#[test]
fn test_basin_size_1() {
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let bitmap: Vec<Vec<u32>> = vec![
        "2199943210".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "3987894921".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "9856789892".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "8767896789".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "9899965678".chars().map(|c| c.to_digit(10).unwrap()).collect(),
    ];
    let i = 0;
    let j = 0;
    let size = basin_size(bitmap, &mut visited, i, j, 2, 10);
    assert_eq!(size, 3);
}

#[test]
fn test_basin_size_2() {
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let bitmap: Vec<Vec<u32>> = vec![
        "2199943210".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "3987894921".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "9856789892".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "8767896789".chars().map(|c| c.to_digit(10).unwrap()).collect(),
        "9899965678".chars().map(|c| c.to_digit(10).unwrap()).collect(),
    ];
    let i = 0;
    let j = 8;
    let size = basin_size(bitmap, &mut visited, i, j, 5, 10);
    assert_eq!(size, 9);
}

fn part1() {
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