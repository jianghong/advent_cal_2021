use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::max;

fn main() {
    part1();
}

fn part1() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());
    let paper_size = 2000;
    let mut paper: Vec<Vec<bool>> = vec![vec![false; paper_size]; paper_size];
    let mut parse_folding = false;
    let mut folding_instructions: Vec<(String, u32)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // check empty line
        if line.len() == 0 {
            parse_folding = true;
            continue;
        }

        if !parse_folding {

            let mut line = line.split(',');
            let x = line.next().unwrap().parse::<usize>().unwrap();
            let y = line.next().unwrap().parse::<usize>().unwrap();
            max_x = max(max_x, x);
            max_y = max(max_y, y);
            paper[y][x] = true;
        } else {
            let mut line = line.split(" ");
            let mut split = line.nth(2).unwrap().split("=");
            let direction = split.next().unwrap();
            let position = split.next().unwrap().parse::<u32>().unwrap();
            folding_instructions.push((direction.to_string(), position));
        }
    }
    paper.truncate(max_y + 1);
    for row in paper.iter_mut() {
        row.truncate(max_x + 1);
    }
    for instruction in folding_instructions[0..1].iter() {
        let (direction, position) = instruction.clone();
        let position = (position) as usize;
        if direction == "x" {
            // merge left and right
            for (i, row) in paper.iter_mut().enumerate() {
                let (left, right) = row.split_at_mut(position);
                // let mut left = left.to_vec();
                let mut right = right[1..].to_vec();
                right.reverse();

                for (j, cell) in left.iter_mut().enumerate() {
                    *cell = right[j] || *cell;
                }
                row.truncate(position);
                // println!("{:?}", row);
            }
        } else if direction =="y" {
            let (top, bottom) = paper.split_at(position);
            let mut top = top.to_vec();
            let mut bottom = bottom[1..].to_vec();
            bottom.reverse();
            // merge top and bottom
            for (i, row) in top.iter_mut().enumerate() {
                for (j, cell) in row.iter_mut().enumerate() {
                    *cell = bottom[i][j] || *cell;
                }
            }
            paper = top; 
        }
    }
    // println!("paper {:?}", paper);

    // count the number of true cells
    let mut dots_visible = 0;
    for (i, row) in paper.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                dots_visible += 1;
                println!("{} {}", i, j);
            }
        }
    }
    println!("part 1 {}", dots_visible);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_vec() {
        let v = false || true;
        assert_eq!(v, true);
    }
}
