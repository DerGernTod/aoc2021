use std::collections::{HashSet, HashMap};
use std::fs;

pub fn part_1() {
    let scanner_data = read_scanner_data("./input/day_19.txt");
    println!("Total unique beacons: {}", calc_total_unique_beacons(scanner_data));
}

pub fn part_2() {
    let coords = read_coords("./input/day_19_scanner_coords.txt");
    println!("Highest manhattan distance: {}", calc_highest_manhattan_distance(&coords));
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

fn read_coords(path: &str) -> Vec<(i32, i32, i32)> {
    let data = fs::read_to_string(path).unwrap();
    data
        .split("\n")
        .map(|scanner_str| {
            let mut iter = scanner_str
                .split(",")
                .map(|coord_str| i32::from_str_radix(coord_str, 10).unwrap());
            (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
        }).collect()
}

fn rotate_id(point: &(i32, i32, i32), rot: usize) -> (i32, i32, i32) {
    match rot {
        0 => (point.0, point.1, point.2),     //[x, y, z]
        1 => (point.0, point.2, -point.1),    //[x, z, -y],
        2 => (point.0, -point.1, -point.2),   //[x, -y, -z],
        3 => (point.0, -point.2, point.1),    //[x, -z, y],
        4 => (point.1, point.0, -point.2),    //[y, x, -z],
        5 => (point.1, point.2, point.0),     //[y, z, x],
        6 => (point.1, -point.0, point.2),    //[y, -x, z],
        7 => (point.1, -point.2, -point.0),   //[y, -z, -x],
        8 => (point.2, point.0, point.1),     //[z, x, y],
        9 => (point.2, point.1, -point.0),    //[z, y, -x],
        10 => (point.2, -point.0, -point.1),  //[z, -x, -y],
        11 => (point.2, -point.1, point.0),   //[z, -y, x],
        12 => (-point.0, point.1, -point.2),  //[-x, y, -z],
        13 => (-point.0, point.2, point.1),   //[-x, z, y],
        14 => (-point.0, -point.1, point.2),  //[-x, -y, z],
        15 => (-point.0, -point.2, -point.1), //[-x, -z, -y],
        16 => (-point.1, point.0, point.2),   //[-y, x, z],
        17 => (-point.1, point.2, -point.0),  //[-y, z, -x],
        18 => (-point.1, -point.0, -point.2), //[-y, -x, -z],
        19 => (-point.1, -point.2, point.0),  //[-y, -z, x],
        20 => (-point.2, point.0, -point.1),  //[-z, x, -y],
        21 => (-point.2, point.1, point.0),   //[-z, y, x],
        22 => (-point.2, -point.0, point.1),  //[-z, -x, y],
        23 => (-point.2, -point.1, -point.0), //[-z, -y, -x],
        _ => unreachable!(),
    }
}

fn get_rotations(coord: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let mut res = vec![*coord; 24];
    for i in 0..res.len() {
        res[i] = rotate_id(coord, i);
    }
    res
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

fn check_matches_per_rotation<'a>(relative_reference_beacons: &'a Vec<Vec<(i32, i32, i32)>>, rotations: &'a Vec<Vec<(i32, i32, i32)>>) -> Vec<(usize, usize, usize)> {
    rotations
    .iter()
    .enumerate()
    .map(|(rot_id, rot_beacons)|
        relative_reference_beacons
        .iter()
        .map(|relative_reference_beacons| 
            rot_beacons
            .iter()
            .enumerate()
            .filter_map(|(rot_beacon_id, rot_beacon)| relative_reference_beacons.iter().position(|coord| coord != &(0i32, 0i32, 0i32) && coord == rot_beacon).and_then(|ref_id| Some((ref_id, rot_beacon_id))))
            .map(|(ref_id, rot_beacon_id)| (rot_id, ref_id, rot_beacon_id))
            .collect::<Vec<(usize, usize, usize)>>()
        )
        .flatten()
        .collect::<Vec<(usize, usize, usize)>>())
    .flatten()
    .collect()
}


fn calc_total_unique_beacons(scanners: Vec<Vec<(i32, i32, i32)>>) -> usize {
    let first_scanner = scanners.get(0).unwrap();
    let mut relative_scanner_locations = vec![None; scanners.len()];
    relative_scanner_locations[0] = Some((0, (0, 0, 0)));
    let mut unique_beacons: HashSet<(i32, i32, i32)> = HashSet::from_iter(first_scanner.clone().into_iter());
    while relative_scanner_locations.iter().flatten().count() < relative_scanner_locations.len() {
        for ref_scanner_id in 0..scanners.len() {
            
            if let Some((rot, offset)) = relative_scanner_locations.get(ref_scanner_id).unwrap() {
                let ref_scanner = scanners
                    .get(ref_scanner_id)
                    .unwrap()
                    .iter()
                    .map(|coord| add(&offset, &rotate_id(coord, *rot)))
                    .collect();    
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
                    let valid_coords_scanner_data: Vec<(usize, usize, usize)> = relative_beacons
                    .iter()
                    .map(|relative_beacons| {
                        // check offset to all other vals
                        let rotations = calc_scanner_rotations(&relative_beacons);
                        check_matches_per_rotation(&relative_reference_beacons, &rotations)
                    })
                    .flatten()
                    .collect();
        
                    let valid_coords_by_rotation = valid_coords_scanner_data.iter().fold(HashMap::new(), |mut map, (rot_id, ref_id, rot_beacon_id)| {
                        map.entry(rot_id).or_insert(vec![]).push((ref_id, rot_beacon_id));
                        map
                    })
                    .into_iter()
                    .filter(|(_, val)| val.len() >= 12)
                    .max_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
                    .and_then(|(rot, list)| { 
                        let hash_set = list.into_iter().fold(HashSet::new(), |mut map, mapping| { map.insert(mapping); map });
                        if hash_set.len() >= 12 {
                            Some((rot, hash_set))
                        } else {
                            None
                        }
                    });
                    if let Some((rot, hash_set)) = valid_coords_by_rotation {
        
                        println!("valid coords for rot {}: \n{:?}", rot, hash_set);
                        let (ref_id, beacon_scanner_id) = hash_set.iter().next().unwrap();
                        let beacon = rotate_id(&scanner[**beacon_scanner_id], *rot);
                        let ref_beacon = ref_scanner[**ref_id];
                        let offset = sub(&ref_beacon, &beacon);
                        println!("offset: {:?}", offset);
                        relative_scanner_locations[scanner_id] = Some((*rot, offset));
                        println!("shared coords: {:?}", scanner.iter().map(|coord| add(&rotate_id(coord, *rot), &offset)).filter(|coord| unique_beacons.contains(coord)).collect::<Vec<(i32, i32, i32)>>());
                        unique_beacons = scanner.iter().map(|coord| add(&offset, &rotate_id(coord, *rot))).chain(unique_beacons.into_iter()).collect();
                        break;
                    }
                }
            } else {
                println!("not using scanner {} as reference since it has no location yet", ref_scanner_id);
                continue;
            }
            
        }
    }
    unique_beacons.iter().for_each(|coord| println!("{:?}", coord));
    unique_beacons.len()
}

fn calc_manhattan_distance(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1) + i32::abs(a.2 - b.2)
}

fn calc_highest_manhattan_distance(coords: &Vec<(i32, i32, i32)>) -> i32 {
    coords.iter().map(|coord| coords.iter().map(|other_coord| calc_manhattan_distance(coord, other_coord)).max().unwrap()).max().unwrap()
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
    fn test_part_2() {
        let scanner_coords = vec![(1105,-1205,1229), (-92,-2380,-20)];
        assert_eq!(calc_highest_manhattan_distance(&scanner_coords), 3621);
    }

    #[test]
    fn test_calc_manhattan_distance() {
        
        assert_eq!(calc_manhattan_distance(&(1105,-1205,1229), &(-92,-2380,-20)), 3621);
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