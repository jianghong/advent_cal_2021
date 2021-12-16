use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::min;
use std::collections::HashMap;

fn main() {
    // part1();
    part2();
}

fn inc(n: u32) -> u32 {
    let mut n = n + 1;
    if n > 9 {
        n = 1
    }
    return n;
}

fn part2() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() ).collect());
    }
    // build 5x5 of grid
    let max_x = grid.len();
    let max_y = grid[0].len();
    let mut grid_5x5: Vec<Vec<u32>> = vec![vec![0; grid[0].len() * 5]; grid.len() * 5];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid_5x5[i][j] = grid[i][j];
            for k in 1..5 {
                // println!("{} {} {} {}", i, j, k, max_y);
                grid_5x5[i][j + (k * max_y)] = inc(grid_5x5[i][j + ((k - 1) * max_y)]);
            }
        }
    }
    let max_y = grid_5x5[0].len();
    for i in 0..max_x {
        for j in 0..max_y {
            for k in 1..5 {
                grid_5x5[i + (k * max_x)][j] = inc(grid_5x5[i + ((k - 1) * max_x)][j]);
            }
        }
    }

    println!("starting path finding..");
    let grid = grid_5x5;
    let end_i = grid.len() - 1;
    let end_j = grid[0].len() - 1;
    let visited_grid = vec![vec![false; end_j + 1]; end_i + 1];
    let curr_path = Vec::new();
    let mut memoized_paths: HashMap<(usize, usize), u64> = HashMap::new();
    find_min_path2(&0, 0, 0, 0, curr_path, &grid, visited_grid, end_i, end_j, None, &mut memoized_paths);
    // println!("memoized paths {:?}", memoized_paths);
    let result = min(memoized_paths.get(&(0, 1)).unwrap(), memoized_paths.get(&(1, 0)).unwrap());
    println!("part 1 {}", result);
}

fn find_min_path2(
    val: &u32,
    i: usize,
    j: usize,
    curr_risk: u64,
    curr_path: Vec<(u32, u32)>,
    grid: &Vec<Vec<u32>>,
    visited_grid: Vec<Vec<bool>>,
    end_i: usize,
    end_j: usize,
    prev_min: Option<u64>,
    memoized_paths: &mut HashMap<(usize, usize), u64>
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
        // println!("found path with risk {}", new_risk);
        memoized_paths.insert((i, j), val.clone() as u64);
        return new_risk;
    }

    if let Some(prev_min) = prev_min {
        if new_risk >= prev_min {
            // println!("pruning.. curr_path {:?}", curr_path);
            return u64::MAX;
        }
    }

    // if new
    let mut visited_grid = visited_grid.clone();
    visited_grid[i][j] = true;

    //  always pick where i and j are greater
    let mut neighbor_vals: Vec<(u32, usize, usize)> = Vec::new();
    // if i != 0 {
    //     neighbor_vals.push((grid[i - 1][j], i - 1, j));
    // }
    if i != end_i {
        neighbor_vals.push((grid[i + 1][j], i + 1, j));
    }
    // if j != 0 {
    //     neighbor_vals.push((grid[i][j - 1], i, j - 1));
    // }
    if j != end_j {
        neighbor_vals.push((grid[i][j + 1], i, j + 1));
    }

    // greed algo, pick the next lowest value to check first
    neighbor_vals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut results = Vec::new();
    let mut min_so_far: Option<u64> = prev_min;
    for neighbor in neighbor_vals.clone() {
        if !visited_grid[neighbor.1][neighbor.2] {
            // before committing to a search, lookup neighbors that have min paths defined in lookup table
            let memoized_risk = memoized_paths.get(&(neighbor.1, neighbor.2));

            let path_risk = if let Some(memoized_risk) = memoized_risk {
                // println!("i {} j {} found memoized risk {}", i, j, memoized_risk);
                if *memoized_risk == u64::MAX {
                    *memoized_risk
                } else {
                    // println!("u64 max {}", u64::MAX);
                    // println!("memoized risk {}", memoized_risk);
                    // println!("curr_risk {}", curr_risk);
                    curr_risk + *memoized_risk
                }
            } else {
                // println!("min_so_far {:?}", min_so_far);
                find_min_path(&grid[neighbor.1][neighbor.2], neighbor.1, neighbor.2,
                    new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j,
                    min_so_far, memoized_paths)
            }
;
            if min_so_far.is_none() || min_so_far.unwrap() > path_risk {
                min_so_far = Some(path_risk);
                // println!("setting min_so_far {} path_risk {}", min_so_far.unwrap(), path_risk);
            }
            results.push(path_risk);
        }
    }

    if results.is_empty() {
        return u64::MAX;
    }
    
    let lowest_result = results.iter().min().unwrap();
    let mut min_memoized_neighbor_risk = u64::MAX;
    for neighbor in neighbor_vals.clone() {
        let t = memoized_paths.get(&(neighbor.1, neighbor.2)).unwrap_or(&u64::MAX);
        if t < &min_memoized_neighbor_risk {
            min_memoized_neighbor_risk = *t;
        }
    }
    let memoized_val = if min_memoized_neighbor_risk == u64::MAX {
        u64::MAX
    } else {
        min_memoized_neighbor_risk  + val.clone() as u64
    };
    memoized_paths.insert((i, j),  memoized_val);

    return *lowest_result;
}

fn part1() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() ).collect());
    }

    let end_i = grid.len() - 1;
    let end_j = grid[0].len() - 1;
    let visited_grid = vec![vec![false; end_j + 1]; end_i + 1];
    let curr_path = Vec::new();
    let mut memoized_paths: HashMap<(usize, usize), u64> = HashMap::new();
    find_min_path(&0, 0, 0, 0, curr_path, &grid, visited_grid, end_i, end_j, None, &mut memoized_paths);
    // println!("memoized paths {:?}", memoized_paths);
    let result = min(memoized_paths.get(&(0, 1)).unwrap(), memoized_paths.get(&(1, 0)).unwrap());
    println!("part 1 {}", result);
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
    prev_min: Option<u64>,
    memoized_paths: &mut HashMap<(usize, usize), u64>
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
        // println!("found path with risk {}", new_risk);
        memoized_paths.insert((i, j), val.clone() as u64);
        return new_risk;
    }

    if let Some(prev_min) = prev_min {
        if new_risk >= prev_min {
            // println!("pruning.. curr_path {:?}", curr_path);
            return u64::MAX;
        }
    }

    // if new
    let mut visited_grid = visited_grid.clone();
    visited_grid[i][j] = true;

    //  always pick where i and j are greater
    let mut neighbor_vals: Vec<(u32, usize, usize)> = Vec::new();
    // if i != 0 {
    //     neighbor_vals.push((grid[i - 1][j], i - 1, j));
    // }
    if i != end_i {
        neighbor_vals.push((grid[i + 1][j], i + 1, j));
    }
    // if j != 0 {
    //     neighbor_vals.push((grid[i][j - 1], i, j - 1));
    // }
    if j != end_j {
        neighbor_vals.push((grid[i][j + 1], i, j + 1));
    }

    // greed algo, pick the next lowest value to check first
    neighbor_vals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut results = Vec::new();
    let mut min_so_far: Option<u64> = prev_min;
    for neighbor in neighbor_vals.clone() {
        if !visited_grid[neighbor.1][neighbor.2] {
            // before committing to a search, lookup neighbors that have min paths defined in lookup table
            let memoized_risk = memoized_paths.get(&(neighbor.1, neighbor.2));

            let path_risk = if let Some(memoized_risk) = memoized_risk {
                // println!("i {} j {} found memoized risk {}", i, j, memoized_risk);
                if *memoized_risk == u64::MAX {
                    *memoized_risk
                } else {
                    // println!("u64 max {}", u64::MAX);
                    // println!("memoized risk {}", memoized_risk);
                    // println!("curr_risk {}", curr_risk);
                    curr_risk + *memoized_risk
                }
            } else {
                // println!("min_so_far {:?}", min_so_far);
                find_min_path(&grid[neighbor.1][neighbor.2], neighbor.1, neighbor.2,
                    new_risk, new_path.clone(), grid, visited_grid.clone(), end_i, end_j,
                    min_so_far, memoized_paths)
            }
;
            if min_so_far.is_none() || min_so_far.unwrap() > path_risk {
                min_so_far = Some(path_risk);
                // println!("setting min_so_far {} path_risk {}", min_so_far.unwrap(), path_risk);
            }
            results.push(path_risk);
        }
    }

    if results.is_empty() {
        return u64::MAX;
    }
    
    let lowest_result = results.iter().min().unwrap();
    let mut min_memoized_neighbor_risk = u64::MAX;
    for neighbor in neighbor_vals.clone() {
        let t = memoized_paths.get(&(neighbor.1, neighbor.2)).unwrap_or(&u64::MAX);
        if t < &min_memoized_neighbor_risk {
            min_memoized_neighbor_risk = *t;
        }
    }
    let memoized_val = if min_memoized_neighbor_risk == u64::MAX {
        u64::MAX
    } else {
        min_memoized_neighbor_risk  + val.clone() as u64
    };
    memoized_paths.insert((i, j),  memoized_val);

    return *lowest_result;
}