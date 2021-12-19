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


fn print_tree(tree: &TreeNode<u32>, depth: u32) {
    let left_pad = " ".repeat(depth as usize);
    println!("{}: {}{:?}", depth, left_pad, tree.literal);
    if tree.left.is_some() {
        print_tree(&tree.left.as_ref().unwrap(), depth + 1);
    }
    if tree.right.is_some(){
        print_tree(&tree.right.as_ref().unwrap(), depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        let mut lc = "[[[[1,1],[2,2]],[3,3]],[4,4]]".chars();
        let tree = parse_line_to_tree(&mut lc);
        println!("{:?}", tree);
        print_tree(&tree, 0);
    }
}