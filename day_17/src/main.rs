use std::ops::Range;
use std::ops::RangeBounds;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input = "target area: x=169..206, y=-108..-68";
    let mut max_y = 0;
    let target_range = parse_target_range(input);
    let highest_initial_launch = find_highest_initial_launch(&target_range);
    println!("Part 1 {:?}", highest_initial_launch);

    let test_target_range = TargetRange {
        x: 20..31, // add 1 to end bound to include the last value
        y: -10..-4 // add 1 to end bound to include the last value
    };
    let reader = BufReader::new(File::open("src/test_answers.txt").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let coords = parse_line(&line);
        for coord in coords {
            let check = check_probe_lands_in_target(0, 0, coord.0, coord.1, &test_target_range, &mut max_y);
            if !check {
                println!("Failed check on initial velocity {:?}", coord);
                return;
            }
        }
    }

    let valid_launch_velocities = find_valid_initial_launch_velocities(&target_range);
    println!("Part 2 {}", valid_launch_velocities.len());
}

struct TargetRange {
    x: Range<i32>,
    y: Range<i32>
}
// The probe's x,y position starts at 0,0. Then, it will follow some trajectory by moving in steps. On each step, these changes occur in the following order:

//     The probe's x position increases by its x velocity.
//     The probe's y position increases by its y velocity.
//     Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
//     Due to gravity, the probe's y velocity decreases by 1.
fn check_probe_lands_in_target(x: i32, y: i32, x_velocity: i32, y_velocity: i32, target_range: &TargetRange, max_y: &mut i32) -> bool {
    let mut x = x;
    let mut y = y;
    let mut x_velocity = x_velocity;
    let mut y_velocity = y_velocity;
    let mut local_max_y = *max_y;

    loop {
        x += x_velocity;
        y += y_velocity;
        x_velocity -= x_velocity.signum();
        y_velocity -= 1;

        if y > local_max_y {
            local_max_y = y;
        }
        if target_range.x.contains(&x) && target_range.y.contains(&y) {
            *max_y = local_max_y;
            return true;
        }
        if x > target_range.x.end || y < target_range.y.start {
            return false;
        }
    }
}

fn find_highest_initial_launch(target_range: &TargetRange) -> (i32, i32, i32) {
    let mut prev_max_y = 0;
    let mut new_max_y = 0;
    let mut max_initial_launch = (0, 0);
    let max_x = target_range.x.start;
    let max_y = target_range.y.start.signum() * target_range.y.start;
    for i in 0..max_x {
        for j in 0..max_y {
            if check_probe_lands_in_target(0, 0, i, j, target_range, &mut new_max_y) {
                if new_max_y > prev_max_y {
                    prev_max_y = new_max_y;
                    max_initial_launch = (i, j);
                }
            }
        }
    }

    return (max_initial_launch.0, max_initial_launch.1, prev_max_y);
}


// target area: x=169..206, y=-108..-68
// TargetRange { x: 169..207, y: -108..-67 }
fn parse_target_range(input: &str) -> TargetRange {
    let target = input[13..].to_string();
    let mut split = target.split(", ");
    let x_range_str = split.next().unwrap()[2..].to_string();
    let y_range_str = split.next().unwrap()[2..].to_string();
    let x_range = x_range_str.split("..").collect::<Vec<&str>>();
    let y_range = y_range_str.split("..").collect::<Vec<&str>>();
    let x_start = x_range[0].parse::<i32>().unwrap();
    let x_end = x_range[1].parse::<i32>().unwrap() + 1;
    let y_start = y_range[0].parse::<i32>().unwrap();
    let y_end = y_range[1].parse::<i32>().unwrap() + 1;

    let target_range = TargetRange {
        x: x_start..x_end,
        y: y_start..y_end,
    };

    return target_range;
}

fn find_valid_initial_launch_velocities(target_range: &TargetRange) -> Vec<(i32, i32)>{
    let mut valid_launch_velocities: Vec<(i32, i32)> = Vec::new();
    let max_x = target_range.x.end;
    let max_y = target_range.y.start.signum() * target_range.y.start;
    let mut new_max_y = 0;
    println!("max_x: {}, start_y: {}, max_y: {} ", max_x, target_range.y.start, max_y);
    for i in 0..max_x {
        for j in target_range.y.start..max_y {
            if i == 23 && j == -10   {
                println!("checking prob with i {} j {}", i, j);
            }
            if check_probe_lands_in_target(0, 0, i, j, target_range, &mut new_max_y) {
                valid_launch_velocities.push((i, j));
            }
        }
    }
    println!("valid_launch_velocities: {:?}", valid_launch_velocities);
    return valid_launch_velocities
}

// 23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
// (23, -10), (25, -9), (27, -5), (29, -6), (22, -6), (21, -7), (9, 0), (27, -7), (24, -5)
fn parse_line(input: &str) -> Vec<(i32, i32)> {
    // split on all whitespace
    let mut split = input.split_whitespace();
    let mut line: Vec<(i32, i32)> = Vec::new();
    while let Some(coords) = split.next() {
        let mut coords_split = coords.split(",");
        let x = coords_split.next().unwrap().parse::<i32>().unwrap();
        let y = coords_split.next().unwrap().parse::<i32>().unwrap();
        line.push((x, y));
    }
    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5";
        let result = parse_line(input);
        let expected = vec![
            (23, -10), (25, -9), (27, -5), (29, -6), (22, -6), (21, -7), (9, 0), (27, -7), (24, -5)
        ];
        assert_eq!(result, expected);
    }
    #[test]
    fn test_check_probe_lands_in_target() {
        let target_range = TargetRange {
            x: 20..31, // add 1 to end bound to include the last value
            y: -10..-4 // add 1 to end bound to include the last value
        };
        let mut max_y = 0;
        assert_eq!(check_probe_lands_in_target(0, 0, 6, 3, &target_range, &mut max_y), true);
        assert_eq!(max_y, 6);

        let mut max_y = 0;
        assert_eq!(check_probe_lands_in_target(0, 0, 9, 0, &target_range, &mut max_y), true);
        assert_eq!(max_y, 0);

        let mut max_y = 0;
        assert_eq!(check_probe_lands_in_target(0, 0, 17, -4, &target_range, &mut max_y), false);
        assert_eq!(max_y, 0);

        let mut max_y = 0;
        assert_eq!(check_probe_lands_in_target(0, 0, 6, 9, &target_range, &mut max_y), true);
        assert_eq!(max_y, 45);

        assert_eq!(check_probe_lands_in_target(0, 0, 7, -1, &target_range, &mut max_y), true);
    }

    #[test]
    fn test_find_highest_initial_launch() {
        let target_range = TargetRange {
            x: 20..31, // add 1 to end bound to include the last value
            y: -10..-4 // add 1 to end bound to include the last value
        };
        let result = find_highest_initial_launch(&target_range);
        assert_eq!(result, (6, 9, 45));
    }

    #[test]
    fn test_parse_target_range() {
        let input = "target area: x=169..206, y=-108..-68";
        let target_range = parse_target_range(input);
        assert_eq!(target_range.x, 169..207);
        assert_eq!(target_range.y, -108..-67);
    }


// In the above example, there are 112 different initial velocity values that meet these criteria:

// 23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
// 25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
// 8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
// 26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
// 20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
// 25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
// 25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
// 8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
// 24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
// 7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
// 23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
// 27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
// 8,-2    27,-8   30,-5   24,-7

    #[test]
    fn test_find_valid_initial_launch_velocities() {
        let target_range = TargetRange {
            x: 20..31, // add 1 to end bound to include the last value
            y: -10..-4 // add 1 to end bound to include the last value
        };
        let result = find_valid_initial_launch_velocities(&target_range);
        assert_eq!(result.len(), 112);
    }

}