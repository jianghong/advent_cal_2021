use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines().next().unwrap().unwrap();
    process(&input, 80);
}

fn part2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines().next().unwrap().unwrap();
    process2(&input, 256);
}

fn parse_fish(input: &str) -> Vec<Fish> {
    input.split(",").map(|i| Fish { time_til_spawn: i.parse().unwrap() }).collect()
}

struct Fish {
    time_til_spawn: u32
}

impl Fish {
    fn next(&mut self) {
        if self.time_til_spawn == 0 {
            self.time_til_spawn = 6
        } else {
            self.time_til_spawn -= 1;
        }
    }
}

fn process(input: &str, days: u32) -> usize {
    let mut fishes: Vec<Fish> = parse_fish(input);

    let mut new_fish_count = 0;
    for i in 0..days {
        for fish in &mut fishes {
            if fish.time_til_spawn == 0 {
                new_fish_count += 1;
            }
            fish.next();
        }
        for j in 0..new_fish_count {
            fishes.push(Fish { time_til_spawn: 8})
        }
        new_fish_count = 0;
    }
    let total = fishes.len();
    println!("part 1 total {}", total);
    return total;
}

fn process2(input: &str, days: u32) -> usize {
    let fishes: Vec<Fish> = input.split(",").map(|i| Fish { time_til_spawn: i.parse().unwrap() }).collect();
    let mut spawn_counts: Vec<u64> = vec![0; 9];
    for fish in fishes {
        spawn_counts[fish.time_til_spawn as usize] += 1
    }

    for _ in 0..days {
        let about_to_spawn_count = spawn_counts[0];
        for i in 0..spawn_counts.len() {
            if i == 6 {
                spawn_counts[i] = spawn_counts[i+1];
                spawn_counts[i] += about_to_spawn_count;
            } else if i == 8 {
                spawn_counts[i] = about_to_spawn_count
            } else {
                spawn_counts[i] = spawn_counts[i+1];
            }
        }
    }

    let total_fish: u64 = spawn_counts.iter().sum();
    println!("total fish {}", total_fish);
    return total_fish as usize
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_process1() {
        let input = "3,4,3,1,2";
        let result = process(input, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_process2() {
        let input = "3,4,3,1,2";
        let result = process2(input, 80);
        assert_eq!(result, 5934);
    }
}