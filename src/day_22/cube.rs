#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Coord(pub i32, pub i32, pub i32);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Cube(pub Coord, pub Coord);

impl Cube {

    pub fn volume(&self) -> u64 {
        let width: u64 = (self.1.0 - self.0.0).try_into().unwrap();
        let height: u64 = (self.1.1 - self.0.1).try_into().unwrap();
        let depth: u64 = (self.1.2 - self.0.2).try_into().unwrap();
        width * height * depth
    }

    pub fn calc_intersection_cube(&self, other: &Cube) -> Option<Cube> {
        let min_a = self.0;
        let min_b = other.0;
        let max_a = self.1;
        let max_b = other.1;
        let x = if is_between(min_a.0, min_b.0, max_b.0) {
            Some((min_a.0, i32::min(max_a.0, max_b.0)))
        } else if is_between(max_a.0, min_b.0, max_b.0) {
            Some((i32::max(min_a.0, min_b.0), max_a.0))
        } else if is_between(min_b.0, min_a.0, max_a.0) {
            Some((min_b.0, i32::min(max_a.0, max_b.0)))
        } else if is_between(max_b.0, min_a.0, max_a.0) {
            Some((i32::max(min_a.0, min_b.0), max_b.0))
        } else if min_a.0 == min_b.0 {
            Some((min_a.0, i32::min(max_a.0, max_b.0)))
        } else {
            None
        };
        let y = if is_between(min_a.1, min_b.1, max_b.1) {
            Some((min_a.1, i32::min(max_a.1, max_b.1)))
        } else if is_between(max_a.1, min_b.1, max_b.1) {
            Some((i32::max(min_a.1, min_b.1), max_a.1))
        } else if is_between(min_b.1, min_a.1, max_a.1) {
            Some((min_b.1, i32::min(max_a.1, max_b.1)))
        } else if is_between(max_b.1, min_a.1, max_a.1) {
            Some((i32::max(min_a.1, min_b.1), max_b.1))
        } else if min_a.1 == min_b.1 {
            Some((min_a.1, i32::min(max_a.1, max_b.1)))
        } else {
            None
        };
        let z = if is_between(min_a.2, min_b.2, max_b.2) {
            Some((min_a.2, i32::min(max_a.2, max_b.2)))
        } else if is_between(max_a.2, min_b.2, max_b.2) {
            Some((i32::max(min_a.2, min_b.2), max_a.2))
        } else if is_between(min_b.2, min_a.2, max_a.2) {
            Some((min_b.2, i32::min(max_a.2, max_b.2)))
        } else if is_between(max_b.2, min_a.2, max_a.2) {
            Some((i32::max(min_a.2, min_b.2), max_b.2))
        } else if min_a.2 == min_b.2 {
            Some((min_a.2, i32::min(max_a.2, max_b.2)))
        } else {
            None
        };

        match (x, y, z) {
            (Some(x), Some(y), Some(z)) => Some(Cube(Coord(x.0, y.0, z.0), Coord(x.1, y.1, z.1))),
            _ => None
        }
    }
}

fn is_between(num: i32, min: i32, max: i32) -> bool {
    num > min && num < max
}

#[cfg(test)]
mod tests {
    use crate::day_22::{Coord, Cube};
    #[test]
    fn test_volume() {
        let cube = Cube(Coord(0, 0, 0), Coord(3, 3, 3));
        assert_eq!(cube.volume(), 27);
    }
    #[test]
    fn test_calc_intersection_cube() {
        let a = Cube(Coord(0, 0, 0), Coord(3, 3, 3));
        let b = Cube(Coord(1, 0, 0), Coord(4, 1, 1));
        let res = a.calc_intersection_cube(&b);
        let reverse = b.calc_intersection_cube(&a);
        assert_eq!(res, reverse);
        let expected = Cube(Coord(1, 0, 0), Coord(3, 1, 1));
        assert_eq!(res.unwrap(), expected);

        assert_eq!(expected.calc_intersection_cube(&expected).unwrap(), expected);

        let b = Cube(Coord(-3, -3, -3), Coord(-1, -1, -1));
        assert_eq!(b.calc_intersection_cube(&a), None);
        assert_eq!(a.calc_intersection_cube(&b), None);

        
        let a = Cube(Coord(0, 3, 0), Coord(3, 6, 1));
        let b = Cube(Coord(2, 2, 0), Coord(6, 7, 1));
        let expected = Cube(Coord(2, 3, 0), Coord(3, 6, 1));
        assert_eq!(a.calc_intersection_cube(&b).unwrap(), expected);
    }
}