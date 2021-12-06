use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::{SyncSender, Receiver};
use std::{thread, time};

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
    let fishes: Vec<Fish> = parse_fish(input);
    let mut total = 0;
    let (tx, rx): (SyncSender<usize>, Receiver<usize>) = sync_channel(1000);

    for fish in fishes {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(calc_lifetime_spawns(fish.time_til_spawn, days)).unwrap();
        });
    }
    drop(tx);
    while let Ok(msg) = rx.recv() {
        total += msg;
    }
    println!("part 2 total {}", total);
    return total;
}

fn calc_lifetime_spawns(initial: u32, days: u32) -> usize {
    let mut t = initial;
    if initial > days  {
        return 1
    }
    let mut next_days = days - initial;
    for _ in 0..initial {
        t = get_next(t);
    }
    if next_days > 0 {
        // spawn
        t = get_next(t);
        next_days -= 1;
        return calc_lifetime_spawns(t, next_days) + calc_lifetime_spawns(t, next_days)
    } else {
        return 1
    }
}

fn get_next(val: u32) -> u32 {
    if val == 0 {
        return 6
    } else {
        return val - 1
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