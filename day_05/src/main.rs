use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
  let filename = "src/input.txt";
  part1(filename);
  part2(filename);
}
fn part1(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut grid: Vec<Vec<i32>> = vec![vec![0]];
  let mut max_x = 1;
  let mut max_y = 1;
  let mut result = 0;
  for (i, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let split: Vec<&str> = line.split(" -> ").collect();
    let start: Vec<&str> = split[0].split(",").collect();
    let first_x = start[0].parse::<i32>().unwrap();
    let first_y = start[1].parse::<i32>().unwrap();
    let end: Vec<&str> = split[1].split(",").collect();
    let second_x = end[0].parse::<i32>().unwrap();
    let second_y = end[1].parse::<i32>().unwrap();
    let start_x = cmp::min(first_x, second_x);
    let end_x = cmp::max(first_x, second_x);
    let start_y = cmp::min(first_y, second_y);
    let end_y = cmp::max(first_y, second_y);

    if (start_x != end_x) && (start_y != end_y) {
      continue
    }

    // find max
    max_x = cmp::max(max_x, cmp::max(start_x, end_x));
    max_y = cmp::max(max_y, cmp::max(start_y, end_y));

    // ensure grid has space
    if (grid[0].len() as i32) <= max_x {
      let diff = max_x - (grid[0].len() as i32) + 1;
      for row in &mut grid {
        let mut grow: Vec<i32> = vec![0; diff as usize];
        row.append(&mut grow);
      }
    }
    if (grid.len() as i32) < max_y {
      let diff = max_y - (grid.len() as i32) + 1;
      let x_size = max_x + 1;
      let mut grow: Vec<Vec<i32>> = vec![vec![0; x_size as usize]; diff as usize];
      grid.append(&mut grow);
    }

    // increment count at coords
    if start_y == end_y {
      for j in start_x..end_x+1 {
        // println!("j {}", j);
        // println!("start_y {}", start_y);
        grid[start_y as usize][j as usize] += 1;
        // if grid[start_y as usize][j as usize] >= 2 {
        //   result += 1;
        // }
      }
    } else {
      for k in start_y..end_y+1 {
        // println!("k {}", k);
        // println!("start_y {}", start_y);
        grid[k as usize][start_x as usize] += 1;
        // if grid[k as usize][start_x as usize] >= 2 {
        //   result += 1;
        // }
      }
    }
    // println!("i {} grid {:?}", i, grid);
  }

  // get result
  for row in grid {
    for column in row {
      if column >= 2 {
        result += 1;
      }
    }
  }
  // println!("grid {:?}", grid);
  println!("result {}", result);

  // println!("grid x {:?}", grid.len());

  // for g in grid {
  //   println!("grid cols {:?}", g.len());
  // }
}

fn part2(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let mut grid: Vec<Vec<i32>> = vec![vec![0]];
  let mut max_x = 1;
  let mut max_y = 1;
  let mut result = 0;
  for (i, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    let split: Vec<&str> = line.split(" -> ").collect();
    let start: Vec<&str> = split[0].split(",").collect();
    let first_x = start[0].parse::<i32>().unwrap();
    let first_y = start[1].parse::<i32>().unwrap();
    let end: Vec<&str> = split[1].split(",").collect();
    let second_x = end[0].parse::<i32>().unwrap();
    let second_y = end[1].parse::<i32>().unwrap();
    let start_x = cmp::min(first_x, second_x);
    let end_x = cmp::max(first_x, second_x);
    let start_y = cmp::min(first_y, second_y);
    let end_y = cmp::max(first_y, second_y);

    // find max
    max_x = cmp::max(max_x, cmp::max(start_x, end_x));
    max_y = cmp::max(max_y, cmp::max(start_y, end_y));

    // ensure grid has space
    if (grid[0].len() as i32) <= max_x {
      let diff = max_x - (grid[0].len() as i32) + 1;
      for row in &mut grid {
        let mut grow: Vec<i32> = vec![0; diff as usize];
        row.append(&mut grow);
      }
    }
    if (grid.len() as i32) <= max_y {
      let diff = max_y - (grid.len() as i32) + 1;
      let x_size = max_x + 1;
      let mut grow: Vec<Vec<i32>> = vec![vec![0; x_size as usize]; diff as usize];
      grid.append(&mut grow);
    }

    let is_diag = (first_x != second_x) && (first_y != second_y);
    if is_diag {
      let mut x_inc = 0;
      let mut y_inc = 0;

      if first_x < second_x {
        x_inc = 1
      } else {
        x_inc = -1
      }
      
      if first_y < second_y {
        y_inc = 1
      } else {
        y_inc = -1
      }

      let inc = (x_inc, y_inc);
      let mut cursor = (first_x, first_y);
      let ending = (second_x, second_y);

      while cursor != ending {
        // println!("cursor {:?} inc {:?}", cursor, inc);
        // println!("grid len {:?}", grid.len());

        // for g in grid {
        //   println!("grid cols {:?}", g.len());
        // }
        grid[cursor.1 as usize][cursor.0 as usize] += 1;
        cursor = (cursor.0 + inc.0, cursor.1 + inc.1);
      }
      grid[cursor.1 as usize][cursor.0 as usize] += 1;
    } else {
      // increment count at coords
      if start_y == end_y {
        for j in start_x..end_x+1 {
          grid[start_y as usize][j as usize] += 1;
        }
      } else {
        for k in start_y..end_y+1 {
          grid[k as usize][start_x as usize] += 1;
        }
      }
    }
    // println!("i {} grid {:?}", i, grid);
  }

  // get result
  for row in grid {
    for column in row {
      if column >= 2 {
        result += 1;
      }
    }
  }
  // println!("grid {:?}", grid);
  println!("result {}", result);

  // println!("grid x {:?}", grid.len());

  // for g in grid {
  //   println!("grid cols {:?}", g.len());
  // }
}