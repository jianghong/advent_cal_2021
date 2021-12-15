use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::min;

fn main() {
    part1();
    // part2();
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c as u8).collect());
    }
    let end_i = grid.len() - 1;
    let end_j = grid[0].len() - 1;
    let visited_grid = vec![vec![false; end_j + 1]; end_i + 1];
    find_min_path(&0, 0, 0, 0, &grid, visited_grid, end_i, end_j);
}

fn find_min_path(
    val: &u8,
    i: usize,
    j: usize,
    curr_risk: u64,
    grid: &Vec<Vec<u8>>,
    visited_grid: Vec<Vec<bool>>,
    end_i: usize,
    end_j: usize
) -> u64
{
    let new_risk = curr_risk + *val as u64;
    if i == end_i && j == end_j {
        return curr_risk;
    }
    let mut visited_grid = visited_grid.clone();
    visited_grid[i][j] = true;

    let mut results: Vec<u64> = Vec::new();
    // check up
    if i != 0 {
        if visited_grid[i - 1][j] == false {
            results.push(
                find_min_path(&grid[i - 1][j], i - 1, j, new_risk, grid, visited_grid.clone(), end_i, end_j)
            );
        }
    }
    // check down
    if i != end_i {
        if visited_grid[i + 1][j] == false {
            results.push(
                find_min_path(&grid[i + 1][j], i + 1, j, new_risk, grid, visited_grid.clone(), end_i, end_j)
            );
        }
    }

    // check left
    if j != 0 {
        if visited_grid[i][j - 1] == false {
            results.push(
                find_min_path(&grid[i][j - 1], i, j - 1, new_risk, grid, visited_grid.clone(), end_i, end_j)
            );
        }
    }

    // check right
    if j != end_j {
        if visited_grid[i][j + 1] == false {
            results.push(
                find_min_path(&grid[i][j + 1], i, j + 1, new_risk, grid, visited_grid.clone(), end_i, end_j)
            );
        }
    }
    println!("results: {:?}", results);
    println!("i {} j {}", i, j);
    return *results.iter().min().unwrap();
}