use std::ops::Range;
use std::ops::RangeBounds;

fn main() {
    println!("Hello, world!");
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
fn check_probe_lands_in_target(x: i32, y: i32, x_velocity: i32, y_velocity: i32, target_range: &TargetRange) -> bool {
    let mut x = x;
    let mut y = y;
    let mut x_velocity = x_velocity;
    let mut y_velocity = y_velocity;

    loop {
        x += x_velocity;
        y += y_velocity;
        x_velocity -= x_velocity.signum();
        y_velocity -= 1;
        println!("x: {}, y: {}", x, y);
        // if x or y is out of target_range
        if target_range.x.contains(&x) && target_range.y.contains(&y) {
            return true;
        }
        // else x or y is greater than target range
        if x > target_range.x.end || y < target_range.y.end {
            return false;
        }
    }
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

        assert_eq!(check_probe_lands_in_target(0, 0, 6, 3, &target_range), true);
        assert_eq!(check_probe_lands_in_target(0, 0, 9, 0, &target_range), true);
        assert_eq!(check_probe_lands_in_target(0, 0, 17, -4, &target_range), false);
    }
}