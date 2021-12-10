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

    // fn decode(&self) -> String {
    //     let mut result = String::new();
    //     for digit in &self.digits {
    //         result.push_str(&digit.decode());
    //     }
    //     result
    // }
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
    
    fn is_three(&self, patterns: Vec<String>) -> bool {
        if self.segments.len() != 5 {
            return false;
        }
        for pattern in &patterns {
            let d = Digit {
                segments: pattern.clone(),
            };
            if d.is_one() {
                // sort string
                let mut sorted = self.segments.clone().chars().collect::<Vec<char>>();
                sorted.sort();
                let sorted_segment = sorted.into_iter().collect::<String>();

                // sort pattern
                let mut sorted_pattern = d.segments.chars().collect::<Vec<char>>();
                sorted_pattern.sort();
                let sorted_pattern = sorted_pattern.into_iter().collect::<String>();
                if contains(&self.segments, pattern) {
                    return true;
                }
            }
        }
        return false;
    }
    fn is_five(&self, patterns: Vec<String>) -> bool {
        if self.segments.len() != 5 {
            return false;
        }
        for pattern in &patterns {
            let d = Digit {
                segments: pattern.clone(),
            };
            if d.is_six(patterns.clone()) {
                // sort string
                let mut sorted = self.segments.clone().chars().collect::<Vec<char>>();
                sorted.sort();
                let sorted_segment = sorted.into_iter().collect::<String>();

                // sort pattern
                let mut sorted_pattern = d.segments.chars().collect::<Vec<char>>();
                sorted_pattern.sort();
                let sorted_pattern = sorted_pattern.into_iter().collect::<String>();

                if contains(pattern, &self.segments) {
                    return true;
                }
            }
        }
        return false;
    }
    fn is_six(&self, patterns: Vec<String>) -> bool {
        if self.segments.len() != 6 {
            return false;
        }
        for pattern in &patterns {
            let d = Digit {
                segments: pattern.clone(),
            };
            if d.is_one() {
                // sort string
                let mut sorted = self.segments.clone().chars().collect::<Vec<char>>();
                sorted.sort();
                let sorted_segment = sorted.into_iter().collect::<String>();

                // sort pattern
                let mut sorted_pattern = d.segments.chars().collect::<Vec<char>>();
                sorted_pattern.sort();
                let sorted_pattern = sorted_pattern.into_iter().collect::<String>();

                if !contains(&self.segments, pattern) {
                    return true;
                }}
        }
        return false;
    }

    fn is_nine(&self, patterns: Vec<String>) -> bool {
        if self.segments.len() != 6 {
            return false;
        }
        for pattern in &patterns {
            let d = Digit {
                segments: pattern.clone(),
            };
            if d.is_five(patterns.clone()) {
                // sort string
                let mut sorted = self.segments.clone().chars().collect::<Vec<char>>();
                sorted.sort();
                let sorted_segment = sorted.into_iter().collect::<String>();

                // sort pattern
                let mut sorted_pattern = d.segments.chars().collect::<Vec<char>>();
                sorted_pattern.sort();
                let sorted_pattern = sorted_pattern.into_iter().collect::<String>();

                if contains(&self.segments, pattern) {
                    return true;
                }
            }
        }
        return false;
    }

    fn decode(&self, patterns: Vec<String>) -> u8 {
        if self.is_one() {
            return 1;
        } else if self.is_four() {
            return 4;
        } else if self.is_seven() {
            return 7;
        } else if self.is_eight() {
            return 8;
        } else if self.segments.len() == 5 {
            if self.is_three(patterns.clone()) {
                return 3;
            } else if self.is_five(patterns.clone()) {
                return 5;
            } else {
                return 2;
            }
        } else if self.segments.len() == 6 {
            if self.is_six(patterns.clone()) {
                return 6;
            } else if self.is_nine(patterns.clone()) {
                return 9;
            } else {
                return 0;
            }
        }
        return 0;
    }
}

fn contains(pattern: &str, segment: &str) -> bool {
    for s in segment.chars() {
        if !pattern.contains(s) {
            return false;
        }
    }
    return true
}

#[test]
fn test_digit_is_digit_1() {
    let digit = Digit { segments: "ab".to_string() };
    assert!(digit.is_digit(1));
}
#[test]
fn test_digit_is_digit_4() {
    let digit = Digit { segments: "abcd".to_string() };
    assert!(digit.is_digit(4));
}
#[test]
fn test_digit_is_digit_7() {
    let digit = Digit { segments: "abc".to_string() };
    assert!(digit.is_digit(7));
}
#[test]
fn test_digit_is_digit_8() {
    let digit = Digit { segments: "abcdefg".to_string() };
    assert!(digit.is_digit(8));
}
#[test]
fn test_digit_decode_2() {
    let digit = Digit { segments: "gcdfa".to_string() };
    let test_pattern = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 2);
}
#[test]
fn test_digit_decode_3() {
    let digit = Digit { segments: "cefdb".to_string() };
    let test_pattern = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 3);
}
#[test]
fn test_digit_decode_5() {
    let digit = Digit { segments: "cdfbe".to_string() };
    let test_pattern = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 5);
}
#[test]
fn test_digit_decode_6() {
    let digit = Digit { segments: "cdfgeb".to_string() };
    let test_pattern = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 6);
}
#[test]
fn test_digit_decode_9() {
    let digit = Digit { segments: "cefabd".to_string() };
    let test_pattern = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 9);
}
#[test]
fn test_digit_decode_0() {
    let digit = Digit { segments: "cagedb".to_string() };
    let test_pattern = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab"
        .split_whitespace()
        .map(|s| s.into())
        .collect();
    assert_eq!(digit.decode(test_pattern), 0);
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut part1_total = 0;
    let mut part2_total = 0;
    for (_, line) in reader.lines().enumerate() {
        let entry = Entry::parse(&line.unwrap());
        let mut num = String::new();
        for digit in entry.digits {
            // if digit is a 1, 4, 7, or 8, add 1 to part1_total
            if digit.is_digit(1) || digit.is_digit(4) || digit.is_digit(7) || digit.is_digit(8) {
                part1_total += 1;
            }
            digit.decode(entry.patterns.clone()).to_string().chars().for_each(|c| num.push(c));
        }
        println!("num {}", num);
        part2_total += num.parse::<u32>().unwrap();
    }
    println!("part1_total: {}", part1_total);
    println!("part1_total: {}", part2_total);
}