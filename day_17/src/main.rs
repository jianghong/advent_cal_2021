use std::ops::Range;
use std::ops::RangeBounds;

fn main() {
    let input = "target area: x=169..206, y=-108..-68";
    let target_range = parse_target_range(input);
    let highest_initial_launch = find_highest_initial_launch(&target_range);
    println!("Part 1 {:?}", highest_initial_launch);
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
        // println!("x: {}, y: {}", x, y);
        if target_range.x.contains(&x) && target_range.y.contains(&y) {
            *max_y = local_max_y;
            return true;
        }
        if x > target_range.x.end || y < target_range.y.end {
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



#[cfg(test)]
mod tests {
    use super::*;

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
}