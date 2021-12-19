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

fn explode(tree: &mut TreeNode<u32>, depth: u32,target_depth: u32) -> (Option<u32>, Option<u32>, bool) {
    if depth >= target_depth {
        if is_literal_pair(tree) {
            let left = tree.left.take().unwrap();
            let right = tree.right.take().unwrap();
            tree.literal = Some(0);
            return (left.literal, right.literal, true);
        } 
    }
    
    let mut left_result: Option<u32> = None;
    let mut right_result: Option<u32> = None;
    let mut exploded = false;
    if tree.left.is_some() && !exploded {
        let result = explode(tree.left.as_mut().unwrap(), depth + 1, target_depth);
        exploded = result.2;
        if result.1.is_some() {
            add_literal_to_left_most_node(tree.right.as_mut().unwrap(), result.1.unwrap());
        }
        left_result = result.0;
    }
    if tree.right.is_some() && !exploded {
        let result = explode(tree.right.as_mut().unwrap(), depth + 1, target_depth);
        exploded = result.2;
        if result.0.is_some() {
            add_literal_to_right_most_node(tree.left.as_mut().unwrap(), result.0.unwrap());
        }
        right_result = result.1;
    }
    return (left_result, right_result, exploded);
}

fn is_literal_pair(tree:&TreeNode<u32>) -> bool {
    if tree.left.is_some() && tree.left.as_ref().unwrap().literal.is_some() &&
       tree.right.is_some() && tree.right.as_ref().unwrap().literal.is_some() {
        return true;
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

fn add_literal_to_right_most_node(tree: &mut TreeNode<u32>, val: u32) {
    if tree.literal.is_some() {
        let mut literal = tree.literal.take().unwrap();
        literal += val;
        tree.literal = Some(literal);
    } else if tree.right.is_some() {
        add_literal_to_right_most_node(&mut tree.right.as_mut().unwrap(), val);
    } else if tree.left.is_some() {
        add_literal_to_right_most_node(&mut tree.left.as_mut().unwrap(), val);
    }
}

fn pt(tree: &TreeNode<u32>) {
    print_tree(tree, 0, true, false);
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
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[3,0],[5,3]],[4,4]],[5,5]]");

        let mut lc = "[[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]],[6,6]]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[[0,[3,2]],[3,3]],[4,4]],[5,5]],[6,6]]");
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[[3,0],[5,3]],[4,4]],[5,5]],[6,6]]");
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[0,[5,3]],[4,4]],[5,5]],[6,6]]");
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[5,0],[7,4]],[5,5]],[6,6]]");

        let mut lc = "[[[[[9,8],1],2],3],4]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[0,9],2],3],4]");

        let mut lc = "[7,[6,[5,[4,[3,2]]]]]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[7,[6,[5,[7,0]]]]");
                
        let mut lc = "[[6,[5,[4,[3,2]]]],1]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[6,[5,[7,0]]],3]");
                
        let mut lc = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
                
        let mut lc = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".chars();
        let mut tree = parse_line_to_tree(&mut lc);
        explode(&mut tree, 0, 4);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn test_parse_tree_to_line() {
        let mut lc = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]".chars();
        let tree = parse_line_to_tree(&mut lc);
        let line = parse_tree_to_line(&tree);
        assert_eq!(line, "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]");
    }

    #[test]
    fn test_add_literal_left() {
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
    fn test_add_literal_right() {
        let mut tree = TreeNode {
            literal: Some(1),
            left: None,
            right: None,
        };
        add_literal_to_right_most_node(&mut tree, 2);
        assert_eq!(tree.literal.unwrap(), 3);

        let mut tree = TreeNode {
            literal: None,
            left: None,
            right: Some(Box::new(TreeNode {
                literal: Some(1),
                left: None,
                right: None,
            })),
        };
        add_literal_to_right_most_node(&mut tree, 2);
        assert_eq!(tree.right.unwrap().literal.unwrap(), 3);

        let mut tree = TreeNode {
            literal: None,
            right: None,
            left: Some(Box::new(TreeNode {
                literal: None,
                left: Some(Box::new(TreeNode {
                    literal: Some(1),
                    left: None,
                    right: None,
                })),
                right: None,
            })),
        };
        add_literal_to_right_most_node(&mut tree, 2);
        assert_eq!(tree.left.unwrap().left.unwrap().literal.unwrap(), 3);
    }

    #[test]
    fn test_parse1() {
        let mut lc = "[[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]".chars();
        let tree = parse_line_to_tree(&mut lc);
        println!("{:?}", tree);
        print_tree(&tree, 0, true, false);
    }

    #[test]
    fn test_is_literal_pair() {
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
        assert_eq!(is_literal_pair(&tree), true);
    }

    
    #[test]
    fn test_is_not_pair() {
        let tree = TreeNode {
            literal: Some(1),
            left: None,
            right: Some(Box::new(TreeNode {
                literal: Some(2),
                left: None,
                right: None,
            })),
        };
        assert_eq!(is_literal_pair(&tree), false);
    }
}