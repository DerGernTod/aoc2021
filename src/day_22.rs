use std::{fs, collections::HashMap};

pub fn part_1() {
    let instructions = parse_instructions("./input/day_22.txt");
    let mut cubes = HashMap::new();
    for instruction in instructions {
        apply_instruction_clamped(&mut cubes, instruction, 50);
    }
    let on_cubes = cubes
        .into_iter()
        .filter(|(_, is_on)| *is_on)
        .count();
    println!("Enabled cubes after instructions: {}", on_cubes);
}

pub fn part_2() {
    let instructions = parse_instructions("./input/day_22.test.2.txt");
    let mut cubes = vec![];
    let len = instructions.len();
    for (id, instruction) in instructions.into_iter().enumerate() {
        println!("starting instruction {}/{}", id + 1, len);
        cubes = apply_instruction(cubes, instruction);
        println!("{}/{} instructions completed", id + 1, len);
    }
    let lights = count_lights(cubes);
    println!("Found {}/{} standalone cubes", lights, len);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord(i32, i32, i32);
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Cube(Coord, Coord);

impl Cube {

    fn volume(&self) -> u64 {
        let width: u64 = (self.1.0 - self.0.0).try_into().unwrap();
        let height: u64 = (self.1.1 - self.0.1).try_into().unwrap();
        let depth: u64 = (self.1.2 - self.0.2).try_into().unwrap();
        width * height * depth
    }
    fn calc_intersection_cube(&self, other: &Cube) -> Option<Cube> {
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

fn parse_instructions(path: &str) -> Vec<(bool, Coord, Coord)> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            line
                .split(' ')
                .fold(None, |acc, val| {
                    if let Some((state, _, _)) = acc {
                        let coords: Vec<(i32, i32)> = val
                            .split(',')
                            .map(|val| {
                                let mut coords_iter = val[2..]
                                    .split("..")
                                    .map(|coord| i32::from_str_radix(coord, 10).expect(&format!("Error parsing {} as number, within {}!", coord, val)));
                                (coords_iter.next().unwrap(), coords_iter.next().unwrap())
                            }).collect();
                        Some((state,
                            Coord(coords[0].0, coords[1].0, coords[2].0),
                            Coord(coords[0].1, coords[1].1, coords[2].1)))
                    } else {
                        Some((val == "on", Coord(0, 0, 0), Coord(0, 0, 0)))
                    }
                }).unwrap()
        })
        .collect()
}

fn apply_instruction_clamped(cubes: &mut HashMap<Coord, bool>, (on, from, to): (bool, Coord, Coord), clamp: i32) {
    if (from.0 < -clamp && to.0 < -clamp)
        || (from.0 > clamp && to.0 > clamp)
        || (from.1 < -clamp && to.1 < -clamp)
        || (from.1 > clamp && to.1 > clamp)
        || (from.2 < -clamp && to.2 < -clamp)
        || (from.2 > clamp && to.2 > clamp) {
        return;
    }
    for x in from.0.clamp(-clamp, clamp)..=to.0.clamp(-clamp, clamp) {
        for y in from.1.clamp(-clamp, clamp)..=to.1.clamp(-clamp, clamp) {
            for z in from.2.clamp(-clamp, clamp)..=to.2.clamp(-clamp, clamp) {
                cubes.insert(Coord(x, y, z), on);
            }
        }
    }
}

fn apply_instruction(cubes: Vec<(Cube, Vec<(Cube, bool)>, bool)>, (on, from, to): (bool, Coord, Coord)) -> Vec<(Cube, Vec<(Cube, bool)>, bool)> {
    let new_cube = Cube(from, to);
    if on {
        let mut res_cubes = vec![];
        let intersections: Vec<(Cube, bool)> = cubes
        .iter()
        .filter_map(|(cube, int, positive)| cube
            .calc_intersection_cube(&new_cube)
            .and_then(|c| {
                let mut res_ops = vec![(c, !positive)];
                res_ops.extend(int
                    .into_iter()
                    .filter_map(|(c, add)| c
                        .calc_intersection_cube(&new_cube)
                        .and_then(|c| Some((c, !add)))));
                Some(res_ops)
            })
        )
        .flatten()
        .collect();
        res_cubes.extend(cubes);
        res_cubes.push((new_cube, intersections, on));
        res_cubes
    } else {
        cubes
        .into_iter()
        .map(|(cube, int, positive)| {
            if let Some(c) = cube.calc_intersection_cube(&new_cube) {
                let mut res_ints = vec![(c, false)];
                for (int, add) in int {
                    res_ints.push((int, add));
                    if let Some(int_int_cube) = int.calc_intersection_cube(&new_cube) {
                        res_ints.push((int_int_cube, !add));
                    }
                }
                (cube, res_ints, positive)
            } else {
                (cube, int, positive)
            }
        })
        .collect()
    }
}

fn neg(n: bool) -> i64 {
    if n { 1 } else { -1 }
}

fn count_lights(operations: Vec<(Cube, Vec<(Cube, bool)>, bool)>) -> i64 {
    operations
        .into_iter()
        .map(|(cube, operations, on)| operations
            .iter()
            .map(|(op_cube, op_on)| op_cube.volume() as i64 * neg(*op_on))
            .sum::<i64>() + cube.volume() as i64 * neg(on)
        ).sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::day_22::*;
    #[test]
    fn test_part_1() {
        let instructions = parse_instructions("./input/day_22.test.txt");
        let mut cubes = HashMap::new();
        for instruction in instructions {
            apply_instruction_clamped(&mut cubes, instruction, 50);
        }
        let on_cubes = cubes
            .into_iter()
            .filter(|(_, is_on)| *is_on)
            .count();
        assert_eq!(on_cubes, 590784);
    }
    #[test]
    fn test_part_2() {
        let instructions = parse_instructions("./input/day_22.test.2.txt");
        let mut cubes = vec![];
        
        for instruction in instructions.into_iter() {
            cubes = apply_instruction(cubes, instruction);
        }
        let light_count: i64 = count_lights(cubes);
        assert_eq!(light_count, 2758514936282235);
    }
    #[test]
    fn test_volume() {
        let cube = Cube(Coord(0, 0, 0), Coord(3, 3, 3));
        assert_eq!(cube.volume(), 27);
    }
    #[test]
    fn test_apply_instructions() {
        let instructions = vec![
            (true, Coord(0, 3, 0), Coord(3, 6, 1)),
            (false, Coord(2, 2, 0), Coord(6, 7, 1)),
            (true, Coord(1, 0, 0), Coord(4, 4, 1)),
            (true, Coord(2, 4, 0), Coord(3, 5, 1)),
        ];
        let mut cubes = vec![];
        for instruction in instructions {
            cubes = apply_instruction(cubes, instruction);
        }
        assert_eq!(count_lights(cubes), 18);
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