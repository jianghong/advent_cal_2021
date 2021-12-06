use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let filename = "src/test.txt";
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let input = reader.lines().next().unwrap().unwrap();
    process(&input);
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

fn process(input: &str) -> usize {
    let days = 80;
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
    let mut fishes: Vec<Fish> = parse_fish(input);
    let mut total = 0;
    for fish in &mut fishes {
        total += calc_lifetime_spawns(fish, days);
    }
    println!("part 2 total {}", total);
    return total;
}

fn calc_lifetime_spawns(fish: &mut Fish, days: u32) -> usize {
    if fish.time_til_spawn > days  {
        return 1
    }
    let mut next_days = days - fish.time_til_spawn;
    for _ in 0..fish.time_til_spawn {
        fish.next();
    }
    if next_days > 0 {
        // spawn
        fish.next();
        next_days -= 1;
        return calc_lifetime_spawns(fish, next_days) + calc_lifetime_spawns(&mut Fish { time_til_spawn: 8 }, next_days)
    } else {
        return 1
    }
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

    #[test]
    fn test_calc_lifetime_spawns() {
        let input = "3,4,3,1,2";
        let result = process2(input, 80);
        assert_eq!(result, 5934);
    }
    
    #[test]
    fn test_calc_lifetime_spawns_simple() {
        let input = "3";
        let result = process2(input, 12);
        assert_eq!(result, 3);
    }
}