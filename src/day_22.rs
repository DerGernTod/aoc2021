use std::{fs, collections::HashMap};
use self::cube::*;
mod cube;

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
    let instructions = parse_instructions("./input/day_22.txt");
    let cubes = instructions.into_iter().fold(vec![], apply_instruction);
    let lights = count_lights(cubes);
    println!("{} lights are lit!", lights);
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

fn update_negative_intersections(c: Cube, int: Vec<(Cube, bool)>) -> Vec<(Cube, bool)>  {
    let mut res_ints = vec![(c, false)];
    for (int, add) in int {
        res_ints.push((int, add));
        if let Some(int_int_cube) = int.calc_intersection_cube(&c) {
            res_ints.push((int_int_cube, !add));
        }
    }
    res_ints
}

fn calc_inverted_intersections(c: Cube, int: &Vec<(Cube, bool)>) -> Vec<(Cube, bool)> {
    let mut res_ops = vec![(c, false)];
    res_ops.extend(int
        .into_iter()
        .filter_map(|(int, add)| int
            .calc_intersection_cube(&c)
            .and_then(|c| Some((c, !add)))));
    res_ops
}

fn apply_instruction(cubes: Vec<(Cube, Vec<(Cube, bool)>)>, (on, from, to): (bool, Coord, Coord)) -> Vec<(Cube, Vec<(Cube, bool)>)> {
    let new_cube = Cube(from, to);
    if on {
        let mut res_cubes = vec![];
        let intersections: Vec<(Cube, bool)> = cubes
        .iter()
        .filter_map(|(cube, int)| cube
            .calc_intersection_cube(&new_cube)
            .and_then(|c| Some(calc_inverted_intersections(c, int)))
        )
        .flatten()
        .collect();
        res_cubes.extend(cubes);
        res_cubes.push((new_cube, intersections));
        res_cubes
    } else {
        cubes
        .into_iter()
        .map(|(cube, int)| {
            if let Some(c) = cube.calc_intersection_cube(&new_cube) {
                (cube, update_negative_intersections(c, int))
            } else {
                (cube, int)
            }
        })
        .collect()
    }
}

fn count_lights(operations: Vec<(Cube, Vec<(Cube, bool)>)>) -> i64 {
    operations
        .into_iter()
        .map(|(cube, operations)| operations
            .iter()
            .map(|(op_cube, op_on)| op_cube.volume() as i64 * if *op_on { 1 } else { -1 })
            .sum::<i64>() + cube.volume() as i64
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
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        let light_count = count_lights(cubes);
        assert_eq!(light_count, 2758514936282235);
    }
    #[test]
    fn test_apply_instructions() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(1, 1, 1)),
            (false, Coord(0, 1, 0), Coord(2, 2, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 4);
    }  
    // the tests below were written without the volume-adaption for 0-width cubes, so just ignore them
        // #[test]  
    fn test_apply_instructions_same() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
            (false, Coord(0, 0, 0), Coord(3, 3, 1)),
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
            (false, Coord(0, 0, 0), Coord(3, 3, 1)),
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 9);
    }
    // #[test]
    fn test_apply_instructions_inside_removed() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
            (false, Coord(1, 1, 0), Coord(2, 2, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 8);
    }
    // #[test]
    fn test_apply_instructions_outside_removed() {
        let instructions = vec![
            (true, Coord(1, 1, 1), Coord(2, 2, 1)),
            (false, Coord(0, 0, 0), Coord(3, 3, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 0);
    }
    // #[test]
    fn test_apply_instructions_inside_overlap() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
            (true, Coord(1, 1, 1), Coord(2, 2, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 9);
    }
    // #[test]
    fn test_apply_instructions_outside_overlap() {
        let instructions = vec![
            (true, Coord(1, 1, 0), Coord(2, 2, 1)),
            (true, Coord(0, 0, 0), Coord(3, 3, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 9);
    }
    // #[test]
    fn test_apply_instructions_side_by_side() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(2, 2, 1)),
            (true, Coord(0, 2, 0), Coord(2, 4, 1)),
            (true, Coord(0, 4, 0), Coord(2, 6, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 12);
    }
    // #[test]
    fn test_apply_instructions_negative_split_overlap() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(2, 2, 1)),
            (true, Coord(0, 4, 0), Coord(2, 6, 1)),
            (false, Coord(0, 1, 0), Coord(1, 5, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 6);
    }    
    // #[test]
    fn test_apply_instructions_negative_split_overlap_twice() {
        let instructions = vec![
            (true, Coord(0, 0, 0), Coord(2, 2, 1)),
            (true, Coord(0, 4, 0), Coord(2, 6, 1)),
            (false, Coord(0, 1, 0), Coord(1, 5, 1)),
            (true, Coord(-1, 1, 0), Coord(1, 5, 1)),
            (false, Coord(0, 1, 0), Coord(1, 5, 1)),
            (false, Coord(0, 1, 0), Coord(1, 2, 1)),
        ];
        let cubes = instructions.into_iter().fold(vec![], apply_instruction);
        assert_eq!(count_lights(cubes), 10);
    }
}