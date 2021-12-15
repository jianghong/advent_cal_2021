use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::min;

fn main() {
    part1();
    // part2();
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() ).collect());
    }

    let end_i = grid.len() - 1;
    let end_j = grid[0].len() - 1;
    let visited_grid = vec![vec![false; end_j + 1]; end_i + 1];
    let curr_path = Vec::new();
    let min_path_risk = find_min_path(&0, 0, 0, 0, curr_path, &grid, visited_grid, end_i, end_j, vec![]);
    println!("part 1 {}", min_path_risk);
}

fn find_min_path(
    val: &u32,
    i: usize,
    j: usize,
    curr_risk: u64,
    curr_path: Vec<(u32, u32)>,
    grid: &Vec<Vec<u32>>,
    visited_grid: Vec<Vec<bool>>,
    end_i: usize,
    end_j: usize,
    prev_results: Vec<u64>
) -> u64
{
    // println!("i {} j {}", i, j);
    let new_risk = curr_risk + *val as u64;
    let mut new_path = curr_path.clone();
    new_path.push((i as u32, j as u32));
    if i == end_i && j == end_j {
        // println!("i {} j {}", i, j);
        // println!("new path {:?}", new_path);
        // println!("curr risk {}", curr_risk);
        // println!("new_risk {}", new_risk);
        // println!("val: {} i {} j {}", val, i, j);
        return new_risk;
    }

    let mut results = prev_results.clone();
    if !results.is_empty() && results.iter().any(|&x| new_risk > x) {
        // println!("results {:?}", results);
        // println!("new_risk {}", new_risk);
        return 9999;
    }

    // if new
    let mut visited_grid = visited_grid.clone();
    visited_grid[i][j] = true;

    // println!("results {:?}", results);
    // println!("i {} j {}", i, j);
    // check up
    if i != 0 {
        if visited_grid[i - 1][j] == false {
            results.push(
                find_min_path(&grid[i - 1][j], i - 1, j, new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j, results.clone())
            );
        }
    }
    // check down
    if i != end_i {
        if visited_grid[i + 1][j] == false {
            results.push(
                find_min_path(&grid[i + 1][j], i + 1, j, new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j, results.clone())
            );
        }
    }

    // check left
    if j != 0 {
        if visited_grid[i][j - 1] == false {
            results.push(
                find_min_path(&grid[i][j - 1], i, j - 1, new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j, results.clone())
            );
        }
    }

    // check right
    if j != end_j {
        if visited_grid[i][j + 1] == false {
            results.push(
                find_min_path(&grid[i][j + 1], i, j + 1, new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j, results.clone())
            );
        }
    }

    if results.is_empty() {
        return 9999;
    }
    
    // println!("visited grid {:?}", visited_grid);
    return *results.iter().min().unwrap();
}