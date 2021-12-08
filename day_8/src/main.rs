use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Entry {
    patterns: Vec<String>,
    digits: Vec<Digit>,
}

impl Entry {
    fn parse(line: &str) -> Entry {
        let split: Vec<Vec<String>> = line.split("|")
            .map(|s| s.trim().split_whitespace().map(|w| w.into()).collect())
            .collect();
        let patterns: Vec<String> = split[0].clone();
        // convert to Digit
        let digits: Vec<Digit> = split[1].iter()
            .map(|s| Digit{ segments: s.into()})
            .collect();
        Entry { patterns, digits }
    }
}

#[derive(Debug)]
struct Digit {
    segments: String,
}

impl Digit {
    fn is_digit(&self, n: u8) -> bool {
        if n == 1 && self.is_one() {
            true
        }
        else if n == 4 && self.is_four() {
            true
        }
        else if n == 7 && self.is_seven() {
            true
        }
        else if n == 8 && self.is_eight() {
            true
        } else {
            false
        }
    }

    fn is_one(&self) -> bool {
        return self.segments.len() == 2;
    }

    fn is_four(&self) -> bool {
        return self.segments.len() == 4;
    }

    fn is_seven(&self) -> bool {
        return self.segments.len() == 3;
    }

    fn is_eight(&self) -> bool {
        return self.segments.len() == 7;
    }
}

// test is_digit
#[test]
fn test_digit_is_digit_1() {
    let digit = Digit { segments: "ab".to_string() };
    assert!(digit.is_digit(1));
}
fn test_digit_is_digit_4() {
    let digit = Digit { segments: "abcd".to_string() };
    assert!(digit.is_digit(4));
}
fn test_digit_is_digit_7() {
    let digit = Digit { segments: "abc".to_string() };
    assert!(digit.is_digit(7));
}
fn test_digit_is_digit_8() {
    let digit = Digit { segments: "abcdefg".to_string() };
    assert!(digit.is_digit(8));
}


fn main() {
    let filename = "src/test.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut part1_total = 0;
    for (_, line) in reader.lines().enumerate() {
        let entry = Entry::parse(&line.unwrap());
        for digit in entry.digits {
            // if digit is a 1, 4, 7, or 8, add 1 to part1_total
            if digit.is_digit(1) || digit.is_digit(4) || digit.is_digit(7) || digit.is_digit(8) {
                part1_total += 1;
            }
        }
    }
    println!("part1_total: {}", part1_total);
}