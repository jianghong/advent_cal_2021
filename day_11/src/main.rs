use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

#[derive(Clone, Copy)]
#[derive(Debug)]
struct Cell {
    val: u32,
    pumped: bool,
}

fn part2() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    // build grid
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(parse_line(&line));
    }
    let mut steps = 1;
    loop {
        grid = visit_cells(grid.clone());
        // flash
        for (i, row) in grid.clone().iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                let mut new_cell = Cell{
                    val: column.val,
                    pumped: false,
                };
                if column.val > 9 {
                    new_cell.val = 0;
                }
                grid[i][j] = new_cell;
            }
        }
        if is_grid_all_zeros(&grid) {
            println!("part 2 {}", steps);
            return;
        }
        steps += 1;
    }
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<Cell>> = Vec::new();

    // build grid
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(parse_line(&line));
    }
    let mut flash_count = 0;
    let steps = 100;
    for s in 0..steps {
        println!("step {}", s);
        grid = visit_cells(grid.clone());
        // flash
        for (i, row) in grid.clone().iter().enumerate() {
            for (j, column) in row.iter().enumerate() {
                let mut new_cell = Cell{
                    val: column.val,
                    pumped: false,
                };
                if column.val > 9 {
                    new_cell.val = 0;
                    flash_count += 1;
                }
                grid[i][j] = new_cell;
            }
        }
        print_grid(grid.clone());
    }
    println!("part 1 {}", flash_count);
}

fn is_grid_all_zeros(grid: &Vec<Vec<Cell>>) -> bool {
    for row in grid.iter() {
        for cell in row.iter() {
            if cell.val != 0 {
                return false;
            }
        }
    }
    true
}

fn parse_line(line: &str) -> Vec<Cell> {
    let mut result = Vec::new();
    for c in line.chars() {
        let my_number = c.to_digit(10).unwrap();
        result.push(Cell { val: my_number, pumped: false });
    }
    result
}

fn visit_cells(grid: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut grid = grid.clone();
    for (i, row) in grid.clone().iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            grid = visit_cell(grid.clone(), i, j);
        }
    }
    grid
}

fn visit_cell(grid: Vec<Vec<Cell>>, i: usize, j: usize) -> Vec<Vec<Cell>>{
    // println!("visiting {} {}", i, j);
    let mut grid = grid.clone();
    let mut cell = grid[i][j];
    cell.val += 1;
    grid[i][j] = cell;
    
    // pump adjacent cells 
    grid = pump_adjacent_cells(&mut grid, i, j);
    grid
}

fn pump_adjacent_cells(grid: &mut Vec<Vec<Cell>>, i: usize, j: usize) -> Vec<Vec<Cell>> {
    let mut grid = grid.clone();
    let mut cell = grid[i][j];
    if cell.val > 9 && !cell.pumped {
        cell.pumped = true;
        grid[i][j] = cell;
        // println!("pumping adjacent to {} {}", i, j);
        if i != 0 {
            grid[i - 1][j].val += 1;
            grid = pump_adjacent_cells(&mut grid, i - 1, j);
        }
        if i != grid.len() - 1 {
            grid[i + 1][j].val += 1;
            grid = pump_adjacent_cells(&mut grid, i + 1, j);
        }
        if j != 0  {
            grid[i][j - 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i, j - 1);
        }
        if j != grid[i].len() - 1 {
            grid[i][j + 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i, j + 1);
        }
        if i != 0 && j != 0 {
            grid[i - 1][j - 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i - 1, j - 1);
        }
        if i != 0 && j != grid[i].len() - 1 {
            grid[i - 1][j + 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i - 1, j + 1);
        }
        if j != 0 && i != grid.len() - 1 {
            grid[i + 1][j - 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i + 1, j - 1);
        }
        if j != grid[i].len() - 1 && i != grid.len() - 1 {
            grid[i + 1][j + 1].val += 1;
            grid = pump_adjacent_cells(&mut grid, i + 1, j + 1);
        }
    }
    grid
}

fn print_grid(grid: Vec<Vec<Cell>>) {
    for row in grid.iter() {
        let mut s = Vec::new();
        for cell in row.iter() {
            s.push(cell.val.to_string());
        }
        println!("{}", s.join(""));
    }
}