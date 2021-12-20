use std::ops::Sub;


fn main() {
    println!("Hello, world!");
}


struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_subtract_coord() {
        let c1 = Coord { x: 2, y: 3, z: 5 };
        let c2 = Coord { x: 5, y: 1, z: -2 };
        let result = c1 - c2;
        assert_eq!(result.x, -3);
        assert_eq!(result.y, 2);
        assert_eq!(result.z, 7);

        let mut input = "
        -618,-824,-621
        -537,-823,-458
        -447,-329,318
        404,-588,-901
        544,-627,-890
        528,-643,409
        -661,-816,-575
        390,-675,-793
        423,-701,434
        -345,-311,381
        459,-707,401
        -485,-357,347".lines();
        let mut next = input.next();
        while next.is_some() {
            println!("{}", next.unwrap().trim());
            next = input.next();
        }
    }
}