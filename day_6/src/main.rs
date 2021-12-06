use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let filename = "src/test.txt";
    part1(filename);
    // part2(filename);
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines().next().unwrap().unwrap();
    process(&input);
}

fn process(input: &str) -> usize {
    let days = 256;
    let mut fishes: Vec<Fish> = input.split(",").map(|i| Fish { time_til_spawn: i.parse().unwrap() }).collect();

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
        println!("day {}", i);
    }
    return fishes.len();
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_example() {
        let input = "3,4,3,1,2";
        let result = process(input);
        assert_eq!(result, 5934);
    }
}