use std::io::{BufRead, BufReader};
use std::fs::File;

mod pair_tree;
use pair_tree::*;

fn main() {
    let sum = add_file("src/input.txt");
    let tree = parse_line_to_tree(&mut sum.chars());
    let mag = magnitude(&tree);
    println!("Part 1 results {}", mag);

    let result = find_max_magnitude("src/input.txt");
    println!("Part 2 results {}", result);
}

fn add_file(path: &str) -> String {
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut sum: String = String::new();
    let result = reader.read_line(&mut sum);
    sum = sum.trim().to_string();
    assert_eq!(result.is_ok(), true);

    for line in reader.lines() {
        let line = line.unwrap();
        sum = add(&sum, &line);
        let mut tree = parse_line_to_tree(&mut sum.chars());
        reduce(&mut tree);
        sum = parse_tree_to_line(&tree);
    }
    return sum
}

fn add(x: &str, y: &str) -> String {
    format!("[{},{}]", x, y)
}

fn magnitude(tree: &TreeNode<u32>) -> u32 {
    let mut sum = 0;
    if is_pair(&tree) {
        let left = tree.left.as_ref().unwrap();
        let right = tree.right.as_ref().unwrap();
        sum += 3 * magnitude(&left) + 2 * magnitude(&right);
    } else {
        sum += tree.literal.as_ref().unwrap();
    }
    return sum;
}

fn find_max_magnitude(path: &str) -> u32 {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut all_numbers: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        all_numbers.push(line);
    }

    let mut max_mag = 0;
    for i in 0..all_numbers.len() {
        for j in 0..all_numbers.len() {
            if i == j {
                continue;
            }
            let a = &all_numbers[i];
            let b = &all_numbers[j];

            let sum = add(a, b);
            let mut tree = parse_line_to_tree(&mut sum.chars());
            reduce(&mut tree);
            let mag = magnitude(&tree);
            if mag > max_mag {
                max_mag = mag;
            }
        }
    }
    return max_mag;
}

fn reduce(tree: &mut TreeNode<u32>) {
    let mut explode_result = true;
    let mut split_result = true;
    while explode_result || split_result {
        // let t = parse_tree_to_line(&tree);
        // println!("{}", t);
        explode_result = explode(tree, 0, 4).2;
        if !explode_result {
            split_result = split(tree);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_find_max_mag() {
        assert_eq!(find_max_magnitude("src/test6.txt"), 3993);
    }

    #[test]
    fn test_test_file_6() {
        let sum = add_file("src/test6.txt");
        let tree = parse_line_to_tree(&mut sum.chars());
        assert_eq!(magnitude(&tree), 4140);
    }

    #[test]
    fn test_magnitude() {
        let tree = parse_line_to_tree(&mut "[[9,1],[1,9]]".chars());
        assert_eq!(magnitude(&tree), 129);

        let tree = parse_line_to_tree(&mut "[[1,2],[[3,4],5]]".chars());
        assert_eq!(magnitude(&tree), 143);

        let tree = parse_line_to_tree(&mut "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".chars());
        assert_eq!(magnitude(&tree), 3488);

        let tree = parse_line_to_tree(&mut "[[[[5,0],[7,4]],[5,5]],[6,6]]".chars());
        assert_eq!(magnitude(&tree), 1137);
    }

    #[test]
    fn test_add_file() {
        assert_eq!(add_file("src/test1.txt"), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(add_file("src/test2.txt"), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(add_file("src/test3.txt"), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(add_file("src/test4.txt"), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(add_file("src/test5.txt"), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_add() {
        assert_eq!(add("[1, 2]", "[3, 4]"), "[[1, 2],[3, 4]]");
    }
}