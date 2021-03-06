use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash)]
struct Cave {
    name: String,
    is_big: bool,
}

impl PartialEq for Cave {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Cave {}

#[derive(Clone, Debug, PartialEq)]
struct Connection {
    cave_a: Cave,
    cave_b: Cave
}

fn main() {
    // part1();
    part2();
}

fn part2() {
    let reader = BufReader::new(File::open("src/input.txt").unwrap());
    let mut caves: Vec<Cave> = Vec::new();
    let mut connections_map: HashMap<Cave, Vec<Cave>> = HashMap::new();
    let start_cave = Cave { name: "start".to_string(), is_big: false };

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let connection_tuples = parse_connection_from_line(&line);
        for connection_tuple in connection_tuples {
            let connections = connections_map.get_mut(&connection_tuple.0);
            if let Some(connection) = connections {
                connection.push(connection_tuple.1);
            } else {
                connections_map.insert(connection_tuple.0.clone(), vec![connection_tuple.1.clone()]);
            }
        }
    }

    let visited_caves: Vec<Cave> = vec![start_cave.clone()];
    let small_cave_visits: HashMap<Cave, u32> = HashMap::new();
    let total = num_paths_to_end_2(start_cave, &connections_map, visited_caves.clone(), small_cave_visits);
    println!("part 2 {}", total);
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut caves: Vec<Cave> = Vec::new();
    let mut connections_map: HashMap<Cave, Vec<Cave>> = HashMap::new();
    let start_cave = Cave { name: "start".to_string(), is_big: false };

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let connection_tuples = parse_connection_from_line(&line);
        for connection_tuple in connection_tuples {
            let connections = connections_map.get_mut(&connection_tuple.0);
            if let Some(connection) = connections {
                connection.push(connection_tuple.1);
            } else {
                connections_map.insert(connection_tuple.0.clone(), vec![connection_tuple.1.clone()]);
            }
        }
    }

    let visited_caves: Vec<Cave> = vec![start_cave.clone()];
    let total = num_paths_to_end(start_cave, &connections_map, visited_caves.clone());
    println!("part 1 {}", total);
}

fn parse_connection_from_line(line: &str) -> Vec<(Cave, Cave)> {
    let split = line.split("-").collect::<Vec<&str>>();
    let a = split[0];
    let b = split[1];

    let mut cave_a = Cave {
        name: a.to_string(),
        is_big: a.chars().next().unwrap().is_uppercase(),
    };
    let mut cave_b = Cave {
        name: b.to_string(),
        is_big: b.chars().next().unwrap().is_uppercase(),
    };
    
    vec![(
        cave_a.clone(),
        cave_b.clone()
    ),
    (
        cave_b.clone(),
        cave_a.clone(),
    )]
}

fn num_paths_to_end(cave: Cave, connections_map: &HashMap<Cave, Vec<Cave>>, visited_caves: Vec<Cave>) -> u64 {
    let mut visited_caves = visited_caves.clone();
    if cave.name == "end" {
        return 1;
    }
    if !cave.is_big {
        visited_caves.push(cave.clone());
    }

    let mut num_paths = 0;
    for connected_cave in connections_map.get(&cave).unwrap() {
        if !visited_caves.contains(connected_cave) {
            num_paths += num_paths_to_end(connected_cave.clone(), connections_map, visited_caves.clone());
        }
    }

    num_paths
}


fn num_paths_to_end_2(cave: Cave, connections_map: &HashMap<Cave, Vec<Cave>>, visited_caves: Vec<Cave>, small_cave_visits: HashMap<Cave, u32>) -> u64 {
    // println!("visiting {:?}", cave);
    // println!("visited {:?}", visited_caves);
    let mut visited_caves = visited_caves.clone();
    let mut small_cave_visits = small_cave_visits.clone();
    // println!("small cave visits {:?}", small_cave_visits);
    if cave.name == "end" {
        return 1;
    }
    if !cave.is_big {
        if cave.name != "start" {
            if small_cave_visits.contains_key(&cave) {
                *small_cave_visits.get_mut(&cave).unwrap() += 1;
                for small_caves in small_cave_visits.keys() {
                    visited_caves.push(small_caves.clone());
                }
            } else {
                small_cave_visits.insert(cave.clone(), 1);
            }
        }
        let max_visits = small_cave_visits.values().max();
        if let Some(max_visits) = max_visits {
            if *max_visits >= 2 {
                visited_caves.push(cave.clone());
            }
        }

    }

    let mut num_paths = 0;
    for connected_cave in connections_map.get(&cave).unwrap() {
        if !visited_caves.contains(connected_cave) {
            num_paths += num_paths_to_end_2(connected_cave.clone(), connections_map, visited_caves.clone(), small_cave_visits.clone());
        }
    }

    num_paths
}