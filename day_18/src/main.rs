use std::io::{BufRead, BufReader};
use std::fs::File;

mod pair_tree;

fn main() {
    println!("Hello, world!");
}

fn add_file(path: &str) -> String{
    let mut reader = BufReader::new(File::open(path).unwrap());
    let mut sum: String = String::new();
    let result = reader.read_line(&mut sum);
    sum = sum.trim().to_string();
    assert_eq!(result.is_ok(), true);

    for line in reader.lines() {
        let line = line.unwrap();
        sum = add(&sum, &line);
    }
    return sum
}

fn add(x: &str, y: &str) -> String {
    format!("[{},{}]", x, y)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add("[1, 2]", "[3, 4]"), "[[1, 2],[3, 4]]");
    }

    #[test]
    fn test_add_file() {
        assert_eq!(add_file("src/test1.txt"), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(add_file("src/test2.txt"), "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
        assert_eq!(add_file("src/test3.txt"), "[[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]],[6,6]]");
    }
}