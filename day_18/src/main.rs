use std::io::{BufRead, BufReader};
use std::fs::File;

mod pair_tree;
use pair_tree::*;

fn main() {
    let sum = add_file("src/input.txt");
    let tree = parse_line_to_tree(&mut sum.chars());
    let mag = magnitude(&tree);
    println!("Part 1 results {}", mag);
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
        let mut explode_result = true;
        let mut split_result = true;
        while explode_result || split_result {
            // let t = parse_tree_to_line(&tree);
            // println!("{}", t);
            explode_result = explode(&mut tree, 0, 4).2;
            if !explode_result {
                split_result = split(&mut tree);
            }
        }
        sum = parse_tree_to_line(&tree);
    }
    return sum
}

fn add(x: &str, y: &str) -> String {
    format!("[{},{}]", x, y)
}

// To check whether it's the right answer, the snailfish teacher only checks the 
// magnitude of the final sum. The magnitude of a pair is 3 times the magnitude of 
// its left element plus 2 times the magnitude of its right element. The magnitude
//  of a regular number is just that number.

// For example, the magnitude of [9,1] is 3*9 + 2*1 = 29; the magnitude of 
// [1,9] is 3*1 + 2*9 = 21. Magnitude calculations are recursive: the magnitude of
//  [[9,1],[1,9]] is 3*29 + 2*21 = 129.

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

#[cfg(test)]

mod tests {
    use super::*;

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