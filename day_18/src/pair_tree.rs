use core::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TreeNode<T> {
    literal: Option<T>,
    left:  Option<Box<TreeNode<T>>>,
    right:  Option<Box<TreeNode<T>>>,
}

// [[[[1,1],[2,2]],[3,3]],[4,4]]
fn parse_line_to_tree(line_chars: &mut Chars<'_>) -> TreeNode<u32> {
    let mut node = TreeNode {
        literal: None,
        left: None,
        right: None,
    };
    let next = line_chars.next();
    if next == Some('[') {

        node.left = Some(Box::new(parse_line_to_tree(line_chars)));
        // moves to the ','
        line_chars.next();

        node.right = Some(Box::new(parse_line_to_tree(line_chars)));

        // moves to the ']'
        line_chars.next();
    } else {
        let literal = next.unwrap().to_digit(10).unwrap();
        node.literal = Some(literal);
    }
    return node;
}

fn parse_tree_to_line(node: &TreeNode<u32>) -> String {
    let mut line = String::new();
    if node.literal.is_some() {
        line.push_str(&node.literal.unwrap().to_string());
    } else {
        line.push('[');
        line.push_str(&parse_tree_to_line(&node.left.as_ref().unwrap()));
        line.push(',');
        line.push_str(&parse_tree_to_line(&node.right.as_ref().unwrap()));
        line.push(']');
    }
    return line;
}


fn print_tree(tree: &TreeNode<u32>, depth: u32, is_root: bool, is_left: bool) {
    let left_pad = "  ".repeat(depth as usize);
    let desc = format!("{}: {} {}{:?}", depth, if is_root { "" } else if is_left { "l" } else { "r" }, left_pad, tree.literal);
    println!("{}", desc);
    if tree.left.is_some() {
        print_tree(&tree.left.as_ref().unwrap(), depth + 1, false, true);
    }
    if tree.right.is_some(){
        print_tree(&tree.right.as_ref().unwrap(), depth + 1, false, false);
    }
}

fn explode(tree: &mut TreeNode<u32>, depth: u32, target_depth: u32) {
    println!("d {}", depth);
    if depth == target_depth - 1 {
        println!("reached target depth {}", depth);
        println!("{:?}", tree);
        if is_pair(tree.left.as_ref()) {
            println!("Found candidate to explode at depth {}", depth - 1);
            // explode left
            let mut left_pair = tree.left.take().unwrap();
            left_pair.left.take().unwrap();
            let left_pair_right = left_pair.right.take().unwrap();
            left_pair.literal = Some(0);
            tree.left = Some(left_pair);
            add_literal_to_left_most_node(tree.right.as_mut().unwrap(), left_pair_right.literal.unwrap());
        }
    } else {
        if tree.left.is_some() {
            explode(tree.left.as_mut().unwrap(), depth + 1, target_depth);
        }
        if tree.right.is_some() {
            explode(tree.right.as_mut().unwrap(), depth + 1, target_depth);
        }
    }
}

fn is_pair(tree: Option<&Box<TreeNode<u32>>>) -> bool {
    if let Some(tree) = tree {
        if tree.left.is_some() && tree.right.is_some() {
            return true;
        }
    }
    return false;
}

fn add_literal_to_left_most_node(tree: &mut TreeNode<u32>, val: u32) {
    if tree.literal.is_some() {
        let mut literal = tree.literal.take().unwrap();
        literal += val;
        tree.literal = Some(literal);
    } else if tree.left.is_some() {
        add_literal_to_left_most_node(&mut tree.left.as_mut().unwrap(), val);
    } else if tree.right.is_some() {
        add_literal_to_left_most_node(&mut tree.right.as_mut().unwrap(), val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let mut lc = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[0,[3,2]],[3,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_parse_tree_to_line() {
        let mut lc = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]".chars();
        let tree = parse_line_to_tree(&mut lc);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_add_literal() {
        let mut tree = TreeNode {
            literal: Some(1),
            left: None,
            right: None,
        };
        add_literal_to_left_most_node(&mut tree, 2);
        assert_eq!(tree.literal.unwrap(), 3);

        let mut tree = TreeNode {
            literal: None,
            left: Some(Box::new(TreeNode {
                literal: Some(1),
                left: None,
                right: None,
            })),
            right: None,
        };
        add_literal_to_left_most_node(&mut tree, 2);
        assert_eq!(tree.left.unwrap().literal.unwrap(), 3);

        let mut tree = TreeNode {
            literal: None,
            right: Some(Box::new(TreeNode {
                literal: None,
                left: Some(Box::new(TreeNode {
                    literal: Some(1),
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            left: None,
        };
        add_literal_to_left_most_node(&mut tree, 2);
        assert_eq!(tree.right.unwrap().left.unwrap().literal.unwrap(), 3);
    }

    #[test]
    fn test_parse1() {
        let mut lc = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]".chars();
        let tree = parse_line_to_tree(&mut lc);
        println!("{:?}", tree);
        print_tree(&tree, 0, true, false);
    }

    #[test]
    fn test_is_pair() {
        let tree = TreeNode {
            literal: None,
            left: Some(Box::new(TreeNode {
                literal: Some(1),
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                literal: Some(2),
                left: None,
                right: None,
            })),
        };
        assert_eq!(is_pair(Some(&Box::new(tree))), true);
    }

    
    #[test]
    fn test_is_not_pair() {
        let tree = TreeNode {
            literal: Some(1),
            left: None,
            right: None
        };
        assert_eq!(!is_pair(Some(&Box::new(tree))), true);
    }
}