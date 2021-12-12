use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Clone, Debug)]
struct Cave {
    name: String,
    is_big: bool,
    connected: Vec<Cave>
}

#[derive(Debug)]
struct Connection {
    cave_a: Cave,
    cave_b: Cave
}

fn main() {
    part1();
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut caves: Vec<Cave> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let connection = parse_connection_from_line(&line);
        caves.push(connection.cave_a.clone());
        caves.push(connection.cave_b.clone());
        connections.push(connection);
    }

    println!("caves {:?}", caves);
    println!("connections {:?}", connections);
}

fn parse_connection_from_line(line: &str) -> Connection {
    let split = line.split("-").collect::<Vec<&str>>();
    let a = split[0];
    let b = split[1];

    let mut cave_a = Cave {
        name: a.to_string(),
        is_big: a.chars().next().unwrap().is_uppercase(),
        connected: Vec::new()
    };
    let mut cave_b = Cave {
        name: b.to_string(),
        is_big: b.chars().next().unwrap().is_uppercase(),
        connected: Vec::new()
    };
    cave_a.connected.push(cave_b.clone());
    cave_b.connected.push(cave_a.clone());
    
    Connection {
        cave_a: cave_a,
        cave_b: cave_b
    }
}