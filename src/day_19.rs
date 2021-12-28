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

fn find_beacon_matches<'a>(relative_reference_beacons: &'a Vec<(i32, i32, i32)>, relative_beacons: &'a Vec<Vec<(i32, i32, i32)>>, ref_beacons: &Vec<(i32, i32, i32)>, beacon_ref_id: usize) -> Option<(usize, usize)> {
    let mut matches: Vec<(usize, usize)> = relative_reference_beacons
        .iter()
        .map(|ref_coord| {
            relative_beacons
                .iter()
                .filter_map(move |beacon_coord| beacon_coord.iter().position(|coord| coord == ref_coord))
                .collect::<Vec<usize>>()
        })
        .enumerate()
        .filter_map(|(coord_id, ref_id_list)| if ref_id_list.len() == 1 { Some((coord_id, ref_id_list[0])) } else { None })
        .collect();
    if matches.len() >= 11 {
        Some(matches.remove(0))
    } else {
        None
    }
}

fn find_relative_matching_beacons<'a>(relative_reference_beacons: &'a Vec<Vec<(i32, i32, i32)>>, relative_beacons: &'a Vec<Vec<(i32, i32, i32)>>, ref_beacons: &Vec<(i32, i32, i32)>) -> Option<(usize, usize)> {
    relative_reference_beacons
    .iter()
    .enumerate()
    .find_map(|(beacon_ref_id, relative_reference_beacons)| {
        let matches = find_beacon_matches(relative_reference_beacons, relative_beacons, ref_beacons, beacon_ref_id)
            .into_iter()
            .fold(HashMap::new(), |mut map, (ref_id, match_id, offset_id)| {
                (*map.entry(ref_id).or_insert(vec![])).push((beacon_id, offset_id));
                map
        });
        if matches.len() >= 12 {
            println!("found {} matches for beacon ref id {}!", matches.len(), beacon_ref_id);
            // take offsets
            if matches.get(&beacon_ref_id).unwrap().len() > 1 {
                matches.into_iter().find_map(|(ref_id, ids)| {
                    if ref_id == beacon_ref_id {
                        None
                    } else {
                        Some((ref_id, ids[0].1))
                    }
                })
            } else {
                matches.iter().for_each(|(ref_id, b_id)| println!("match at {:?} for ids {}, {:?}", ref_beacons.get(*ref_id).unwrap(), ref_id, b_id));
                matches.into_iter().find_map(|(ref_id, ids)| if ref_id == beacon_ref_id {
                    Some((ref_id, ids[0].0))
                } else {
                    None
                })
            }
        } else {
            None
        }
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
        println!("reference beacons: ");
        ref_scanner.iter().enumerate().for_each(|(id, beacon)| println!(" {}: {:?}", id, beacon));
        for scanner_id in 0..scanners.len() {
            let scanner = scanners.get(scanner_id).unwrap();
            if let Some(t) = relative_scanner_locations.get(scanner_id).unwrap() {
                println!("skipping because id {} has a location: {:?}", scanner_id, t);
                continue;
            }
            let relative_reference_beacons = calc_relative_beacons(&ref_scanner);
            let rotations = calc_scanner_rotations(&scanner);
            // check all permutations for this scanner
            let valid_coords_scanner_data = rotations.iter().enumerate().find_map(|(rotation_id, rotated_beacons)| {
                if scanner_id == 2 && rotation_id == 18 {
                    println!("in");
                }
                println!("checking scanner {} with rotation {}", scanner_id, rotation_id);
                // check offset to all other vals
                let relative_beacons = calc_relative_beacons(&rotated_beacons);
                let relative_beacons_with_ref_index = find_relative_matching_beacons(&relative_reference_beacons, &relative_beacons, &ref_scanner);
                if let Some((ref_id, beacon_id)) = relative_beacons_with_ref_index {
                    println!("found overlaps with scanner {} in rotation {}", scanner_id, rotation_id);
                    let reference_offset = ref_scanner.get(ref_id).unwrap();
                    let scanner_ref_coord = rotated_beacons.get(beacon_id).unwrap();
                    let do_add = rotation_id > 14;
                    let abs_scanner_location = if do_add { add(&reference_offset, &scanner_ref_coord) } else { sub(&reference_offset, &scanner_ref_coord) };
                    if scanner_id == 2 {
                        rot_ops.iter().enumerate().for_each(|(i, ops)| println!("abs scanner loc with rot {}: {:?}", i, ops.iter().rfold(abs_scanner_location, |coord, (deg, axis)| rotate(*deg, *axis, &coord))));
                    }
                    println!("determined scanner location of scanner {} at {:?}, rot ops {:?}", scanner_id, abs_scanner_location, calc_rotations().get(rotation_id));
                    Some((scanner_id,
                        rotation_id,
                        abs_scanner_location,
                        rotated_beacons.into_iter().map(move |coord| if do_add { sub(coord, &abs_scanner_location) } else { add(coord, &abs_scanner_location) }).collect::<Vec<(i32, i32, i32)>>()))
                } else {
                    None
                }
            });
            if let Some((scanner_id, rotation_id, abs_scanner_location, valid_coords)) = valid_coords_scanner_data {
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