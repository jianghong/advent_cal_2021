use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let filename = "src/input.txt";
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut total = 0;
    for (i, line) in reader.lines().enumerate() {
        let val = line.unwrap();
        let illegal_char = find_illegal_char(&val);
        println!("illegal char {}", illegal_char);
        if illegal_char == ')' {
            total += 3;
        } else if illegal_char == ']' {
            total += 57;
        } else if illegal_char == '}' {
            total += 1197
        } else if illegal_char == '>' {
            total += 25137
        } 
    }
    println!("part1 {}", total);
}

fn part2() {
    let filename = "src/input.txt";
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut scores: Vec<u64> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let val = line.unwrap();
        let illegal_char = find_illegal_char(&val);
        if illegal_char == ' ' {
            // incomplete line
            let closing_chunks = find_closing_chunks(&val);
            let mut chunk_total = 0;
            for chunk in closing_chunks.chars() {
                chunk_total *= 5;
                if chunk == ')' {
                    chunk_total += 1;
                } else if chunk == ']' {
                    chunk_total += 2;
                } else if chunk == '}' {
                    chunk_total += 3
                } else if chunk == '>' {
                    chunk_total += 4
                }
            }
            scores.push(chunk_total);
        }
    }
    let middle = scores.len() / 2;
    scores.sort();
    // get middle element of scores
    let middle_score = scores[middle];
    println!("part2 {}", middle_score);
}


// find illegal character 
fn find_illegal_char(input: &str) -> char {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' {
            if stack.pop().unwrap() != '(' {
                return c;
            }
        } else if c == ']' {
            if stack.pop().unwrap() != '[' {
                return c;
            }
        } else if c == '}' {
            if stack.pop().unwrap() != '{' {
                return c;
            }
        } else if c == '>' {
            if stack.pop().unwrap() != '<' {
                return c;
            }
        }
    }
    return ' ';
}


// test find_illegal_char
#[test]
fn test_find_illegal_char() {
    assert_eq!(find_illegal_char("(()]"), ']');
    assert_eq!(find_illegal_char("{([(<{}[<>[]}>{[]{[(<()>"), '}');
    assert_eq!(find_illegal_char("[[<[([]))<([[{}[[()]]]"), ')');
    assert_eq!(find_illegal_char("[<>({}){}[([])<>]]"), ' ');
}

fn find_closing_chunks(input: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    for c in input.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' || c == '>' {
            stack.pop();
        }
    }
    let mut result = String::new();
    while stack.len() > 0 {
        let pop = stack.pop().unwrap();
        if pop == '(' {
            result.push(')');
        } else if pop == '[' {
            result.push(']')
        } else if pop == '{' {
            result.push('}')
        } else if pop == '<' {
            result.push('>')
        }
    }
    return result;
}

// test find_closing_chunks
#[test]
fn test_find_closing_chunks() {
    assert_eq!(find_closing_chunks("[({(<(())[]>[[{[]{<()<>>"), "}}]])})]");
    assert_eq!(find_closing_chunks("[(()[<>])]({[<{<<[]>>("), ")}>]})");
}