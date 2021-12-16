use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use min_max_heap::MinMaxHeap;

fn main() {
    part1();
}

fn part1() {
    let reader = BufReader::new(File::open("src/test.txt").unwrap());
    let mut grid: Vec<Vec<u32>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() ).collect());
    }

    let adj_list = build_adj_list(&grid);
    let mut risk_grid: Vec<Vec<u32>> = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    risk_grid[0][0] = 0;
    let mut heap = init_min_heap();
    while !heap.is_empty() {
        let u = heap.pop_min().unwrap();
        for v in &mut adj_list[&(u.i, u.j)].iter() {
            let new_risk = risk_grid[u.i][u.j] + grid[v.i][v.j];
            if risk_grid[v.i][v.j] > new_risk {
                risk_grid[v.i][v.j] = new_risk;
                heap.push(Node { i: v.i, j: v.j, risk: new_risk });
            }
        }
    }
    println!("part 1 {}", risk_grid[grid.len() - 1][grid[0].len() - 1]);
}


fn build_adj_list(grid: &Vec<Vec<u32>>) -> HashMap<(usize, usize), Vec<Node>> {
    let mut adj_list: HashMap<(usize, usize), Vec<Node>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut adj_nodes = Vec::new();
            if i > 0 {
                adj_nodes.push(
                    Node{
                        i: i - 1,
                        j: j,
                        risk: grid[i][j] + grid[i - 1][j]
                    }
                );
            }
            if i < grid.len() - 1 {
                adj_nodes.push(
                    Node{
                        i: i + 1,
                        j: j,
                        risk: grid[i][j] + grid[i + 1][j]
                    }
                );
            }
            if j > 0 {
                adj_nodes.push(
                    Node{
                        i: i,
                        j: j - 1,
                        risk: grid[i][j] + grid[i][j - 1]
                    }
                );
            }
            if j < grid[0].len() - 1 {
                adj_nodes.push(
                    Node{
                        i: i,
                        j: j + 1,
                        risk: grid[i][j] + grid[i][j + 1]
                    }
                );
            }
            adj_list.insert((i, j), adj_nodes);
        }
    }
    adj_list
}

fn init_min_heap() -> MinMaxHeap<Node> {
    let mut heap = MinMaxHeap::<Node>::new();
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         let risk = if i == 0 && j == 0 {
    //             0
    //         } else {
    //             u32::MAX
    //         };
    //         heap.push(Node { i, j, risk });
    //     }
    // }
    heap.push(Node { i: 0, j: 0, risk: 0 });
    heap
}

#[derive(PartialEq, PartialOrd, Eq, Debug)]
struct Node {
    i: usize,
    j: usize,
    risk: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        self.risk.cmp(&other.risk)
    }
}