use std::collections::{HashSet};
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

fn check_matches_per_rotation<'a>(relative_reference_beacons: &'a Vec<Vec<(i32, i32, i32)>>, rotations: &'a Vec<Vec<(i32, i32, i32)>>) -> Option<(usize, Vec<(usize, usize)>)> {
    rotations
    .iter()
    .enumerate()
    .find_map(|(rot_id, rot_beacons)| {
        println!("checking rotation id {}", rot_id);
        relative_reference_beacons.iter().find_map(|relative_reference_beacons| {
            let matches: Vec<(usize, usize)> = rot_beacons
            .iter()
            .enumerate()
            .filter_map(|(rot_beacon_id, rot_beacon)| {
                if let Some(ref_id) = relative_reference_beacons.iter().position(|coord| coord == rot_beacon) {
                    if rot_beacon != &(0i32, 0i32, 0i32) {
                        println!("found match {}, {}", ref_id, rot_beacon_id);
                    }
                    Some((ref_id, rot_beacon_id))
                } else {
                    None
                }
            }).collect();
            if matches.len() >= 12 {
                Some((rot_id, matches))
            } else {
                None
            }
        })
    })
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
            let valid_coords_scanner_data = relative_beacons.iter().enumerate().find_map(|(rel_beacon_id, relative_beacons)| {
                println!("checking relative coords {}", rel_beacon_id);
                // check offset to all other vals
                let mut rotations = calc_scanner_rotations(&relative_beacons);
                let matches_per_rotation = check_matches_per_rotation(&relative_reference_beacons, &rotations);
                if let Some((rot_id, matches)) = matches_per_rotation {
                    println!("got {} matches in rotation {} for scanner {} in relation to beacon {}", matches.len(), rot_id, scanner_id, rel_beacon_id);
                    let lookup_ref_id = matches.iter().find_map(|(ref_id, rot_id)| if *rot_id == rel_beacon_id { Some(ref_id) } else { None }).unwrap();
                    let relative_offset = ref_scanner[*lookup_ref_id];

                    let (reference_id, rotated_id) = matches.iter().next().unwrap();
                    let root = sub(&ref_scanner[*reference_id], &rotations[rot_id][*rotated_id]);
                    let offset = &scanner[rel_beacon_id];
                    let offset = rot_ops.get(rot_id).unwrap().iter().rfold(*offset, |coord, (deg, axis)| rotate(-*deg, *axis, &coord));
                    let scanner_location = sub(&root, &offset);
                    println!("offset is {:?}, root is {:?}, scanner location is {:?}", offset, root, scanner_location);
                    matches
                            .iter()
                            .map(|(reference_id, rotated_id)| (&ref_scanner[*reference_id], &rotations[rot_id][*rotated_id], reference_id, rotated_id))
                            .for_each(|(ref_beacon, rot_beacon, reference_id, rotated_id)| println!(" {:?} - {:?} at ids {} {}", ref_beacon, rot_beacon, reference_id, rotated_id));
                    Some((scanner_id, rot_id, scanner_location, relative_offset, rotations.remove(rot_id)))
                } else {
                    None
                }
               
            });
            if let Some((scanner_id, rotation_id, abs_scanner_location, relative_offset, valid_coords)) = valid_coords_scanner_data {
                let valid_coords: Vec<(i32, i32, i32)> = valid_coords.into_iter().map(|coord| add(&coord, &relative_offset)).collect();
                println!("added coords: {:?}", valid_coords.iter().filter(|coord| !unique_beacons.contains(coord)).collect::<Vec<&(i32, i32, i32)>>());
                
                unique_beacons = valid_coords.into_iter().chain(unique_beacons.into_iter()).collect();
                scanner_location_todos += 1;
                relative_scanner_locations[scanner_id] = Some((rotation_id, abs_scanner_location));
                println!("found {} unique beacons after scanner iteration for scanner id {}: {:?}", unique_beacons.len(), scanner_id, unique_beacons);
                break;
            }
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