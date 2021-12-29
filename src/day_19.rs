use std::collections::{HashSet, HashMap};
use std::fs;
use std::f32::consts::PI;

pub fn part_1() {
    let scanner_data = read_scanner_data("./input/day_19.test.txt");
    println!("Total unique beacons: {}", calc_total_unique_beacons(scanner_data));
}

pub fn part_2() {

}

fn read_scanner_data(path: &str) -> Vec<Vec<(i32, i32, i32)>> {
    let data = fs::read_to_string(path).unwrap();
    data
        .split("\n\n")
        .map(|scanner_str| scanner_str
            .split("\n")
            .skip(1)
            .map(|beacon_str| {
                let mut iter = beacon_str
                    .split(",")
                    .map(|coord_str| i32::from_str_radix(coord_str, 10).unwrap());
                (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
            }).collect()
        ).collect()
}

fn rotate(deg: f32, axis: usize, (x, y, z): &(i32, i32, i32)) -> (i32, i32, i32) {
    match axis {
        0 => (
            *x,
            y * f32::cos(deg).round() as i32 - z * f32::sin(deg).round() as i32,
            y * f32::sin(deg).round() as i32 + z * f32::cos(deg).round() as i32,
        ),
        1 => (
            x * f32::cos(deg).round() as i32 + z * f32::sin(deg).round() as i32,
            *y,
            x * f32::sin(deg).round() as i32 + z * f32::cos(deg).round() as i32,
        ),
        2 => (
            x * f32::cos(deg).round() as i32 - y * f32::sin(deg).round() as i32,
            x * f32::sin(deg).round() as i32 + y * f32::cos(deg).round() as i32,
            *z),
        _ => panic!("Cannot rotate in more than 3 dimensions!")
    }
}

fn calc_rotations() -> Vec<Vec<(f32, usize)>> {
    let mut res = vec![];
    for y in 0..4 {
        for z in 0..4 {
            let cur_rot = vec![
                (PI * 0.5 * y as f32, 1),
                (PI * 0.5 * z as f32, 2)
            ];
            res.push(cur_rot);
        }
    }
    for z in 0..4 {
        let cur_rot = vec![
            (PI * 0.5 as f32, 0),
            (PI * 0.5 * z as f32, 2)
        ];
        res.push(cur_rot);
    }
    for z in 0..4 {
        let cur_rot = vec![
            (-PI * 0.5 as f32, 0),
            (PI * 0.5 * z as f32, 2)
        ];
        res.push(cur_rot);
    }
    res
}

fn get_rotations(coords: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    calc_rotations()
        .into_iter()
        .map(|ops| ops
            .into_iter()
            .fold(*coords, |coords, (deg, axis)| rotate(deg, axis, &coords)))
        .collect()
}

fn sub(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn add(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> (i32, i32, i32) {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn calc_relative_beacons(references: &Vec<(i32, i32, i32)>) -> Vec<Vec<(i32, i32, i32)>> {
    references
        .iter()
        .map(|beacon| references
            .iter()
            .map(|other| sub(other, beacon))
            .collect())
        .collect()
}

fn calc_scanner_rotations(scanner: &Vec<(i32, i32, i32)>) -> Vec<Vec<(i32, i32, i32)>> {
    scanner
        .into_iter()
        .map(|val| get_rotations(&val))
        .fold(vec![], |mut res_vec, cur_coord_perms| {
            cur_coord_perms.iter().enumerate().for_each(|(i, perm_coord)| {
                if res_vec.get(i) == None {
                    res_vec.push(vec![]);
                }
                res_vec[i].push(*perm_coord);
            });
            res_vec
        })
}

fn check_matches_per_rotation<'a>(relative_reference_beacons: &'a Vec<Vec<(i32, i32, i32)>>, rotations: &'a Vec<Vec<(i32, i32, i32)>>) -> Vec<(usize, usize, usize, usize)> {
    rotations
    .iter()
    .enumerate()
    .map(|(rot_id, rot_beacons)|
        relative_reference_beacons
        .iter()
        .enumerate()
        .map(|(lookup_id, relative_reference_beacons)| 
            rot_beacons
            .iter()
            .enumerate()
            .filter_map(|(rot_beacon_id, rot_beacon)| relative_reference_beacons.iter().position(|coord| coord != &(0i32, 0i32, 0i32) && coord == rot_beacon).and_then(|ref_id| Some((ref_id, rot_beacon_id))))
            .map(|(ref_id, rot_beacon_id)| (rot_id, lookup_id, ref_id, rot_beacon_id))
            .collect::<Vec<(usize, usize, usize, usize)>>()
        )
        .flatten()
        .collect::<Vec<(usize, usize, usize, usize)>>())
    .flatten()
    .collect()
}


fn calc_total_unique_beacons(scanners: Vec<Vec<(i32, i32, i32)>>) -> usize {
    let first_scanner = scanners.get(0).unwrap();
    let rot_ops = calc_rotations();
    let mut relative_scanner_locations = vec![None; scanners.len()];
    relative_scanner_locations[0] = Some((0, (0, 0, 0)));
    let mut scanner_location_todos = scanners.len() - 1;
    let mut unique_beacons: HashSet<(i32, i32, i32)> = HashSet::from_iter(first_scanner.clone().into_iter());
    while scanner_location_todos > 0 {
        scanner_location_todos -= 1;
        let ref_scanner: Vec<(i32, i32, i32)> = unique_beacons.clone().into_iter().collect();
        for scanner_id in 0..scanners.len() {
            let scanner = scanners.get(scanner_id).unwrap();
            if let Some(t) = relative_scanner_locations.get(scanner_id).unwrap() {
                println!("skipping because id {} has a location: {:?}", scanner_id, t);
                continue;
            }
            println!("-- checking scanner {} --", scanner_id);
            let relative_reference_beacons = calc_relative_beacons(&ref_scanner);
            let relative_beacons = calc_relative_beacons(&scanner);
            // check all permutations for this scanner
            let valid_coords_scanner_data: Vec<(usize, usize, usize, usize, usize)> = relative_beacons
            .iter()
            .enumerate()
            .map(|(rel_beacon_id, relative_beacons)| {
                // check offset to all other vals
                let rotations = calc_scanner_rotations(&relative_beacons);
                check_matches_per_rotation(&relative_reference_beacons, &rotations)
                .into_iter()
                .map(|(rot_id, lookup_id, ref_id, rot_beacon_id)| (rel_beacon_id, rot_id, lookup_id, ref_id, rot_beacon_id))
                .collect::<Vec<(usize, usize, usize, usize, usize)>>()
            })
            .flatten()
            .collect();

            let valid_coords_by_rotation = valid_coords_scanner_data.iter().fold(HashMap::new(), |mut map, (rel_beacon_id, rot_id, lookup_id, ref_id, rot_beacon_id)| {
                map.entry(rot_id).or_insert(vec![]).push((rel_beacon_id, lookup_id, ref_id, rot_beacon_id));
                map
            })
            .into_iter()
            .filter(|(_, val)| val.len() >= 12)
            .max_by(|(_, a), (_, b)| a.len().cmp(&b.len()));
            if let Some((rot, list)) = valid_coords_by_rotation {
                // println!("valid coords for rot {}: \n{:?}", rot, list);
                let rotations = calc_scanner_rotations(&scanner);
                let correctly_rotated_scanner = rotations.get(*rot).unwrap();
                let relative_beacons = calc_relative_beacons(correctly_rotated_scanner);
                let v: Vec<Vec<(usize, usize)>> = relative_beacons
                .iter()
                .map(|relative_beacons| relative_reference_beacons
                    .iter()
                    .map(|relative_reference_beacons| relative_reference_beacons
                        .iter()
                        .enumerate()
                        .filter_map(|(rel_ref_id, coord)|
                            if let Some(coord_id) = relative_beacons.iter().position(|rel_coord| rel_coord != &(0i32, 0i32, 0i32) && rel_coord == coord) {
                                Some((rel_ref_id, coord_id))
                            } else {
                                None
                            }
                        )
                        .collect::<Vec<(usize, usize)>>()
                    )
                    .flatten()
                    .collect::<Vec<(usize, usize)>>()
                ).collect();
                
                println!("valid coords for rot {}: \n{:?}", rot, v);
            }


            // if let Some((scanner_id, rotation_id, abs_scanner_location, relative_offset, valid_coords)) = valid_coords_scanner_data {
            //     let valid_coords: Vec<(i32, i32, i32)> = valid_coords.into_iter().map(|coord| add(&coord, &relative_offset)).collect();
            //     println!("added coords: {:?}", valid_coords.iter().filter(|coord| !unique_beacons.contains(coord)).collect::<Vec<&(i32, i32, i32)>>());
                
            //     unique_beacons = valid_coords.into_iter().chain(unique_beacons.into_iter()).collect();
            //     scanner_location_todos += 1;
            //     relative_scanner_locations[scanner_id] = Some((rotation_id, abs_scanner_location));
            //     println!("found {} unique beacons after scanner iteration for scanner id {}: {:?}", unique_beacons.len(), scanner_id, unique_beacons);
            //     break;
            // }
        }
    }
    unique_beacons.iter().for_each(|coord| println!("{:?}", coord));
    unique_beacons.len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::day_19::*;
    #[test]
    fn test_part_1() {
        let scanner_data = read_scanner_data("./input/day_19.test.txt");
        println!("{:?}", scanner_data);
        assert_eq!(calc_total_unique_beacons(scanner_data), 79);
    }
    #[test]
    fn test_rotate() {
        let beacon = (1, 2, 3);
        let permutations = get_rotations(&beacon);
        assert_eq!(permutations.len(), 24);
        let perm_set: HashSet<(i32, i32, i32)> = HashSet::from_iter(permutations);
        assert_eq!(perm_set.len(), 24);
        perm_set.iter().for_each(|c| println!("{:?}", c));
    }
    #[test]
    fn test_calc_relative_vectors() {
        let beacons = vec![(1, 1, 1), (2, 2, 2), (3, 3, 3)];
        let expected = vec![
            vec![(0, 0, 0), (1, 1, 1), (2, 2, 2)],
            vec![(-1, -1, -1), (0, 0, 0), (1, 1, 1)],
            vec![(-2, -2, -2), (-1, -1, -1), (0, 0, 0)]
        ];
        let rel = calc_relative_beacons(&beacons);
        assert_eq!(rel, expected);
    }
}