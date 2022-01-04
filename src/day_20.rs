use std::{fs, collections::HashMap};

pub fn part_1() {
    let (algo, image) = read_input("./input/day_20.txt");
    let min = 0;
    let max = f32::sqrt(image.len() as f32).floor() as i32 - 1;
    let new_map = apply_algo(image, &algo, min, max, false);
    let new_map = apply_algo(new_map, &algo, min - 1, max + 1, true);
    println!("Result image: ");
    print(&new_map, min - 2, max + 2);
    println!("There are {} lights!", count_lights(&new_map));
}

pub fn part_2() {
    let (algo, image) = read_input("./input/day_20.txt");
    let min = 0;
    let max = f32::sqrt(image.len() as f32).floor() as i32 - 1;

    let mut new_map = apply_algo(image, &algo, min, max, false);
    for i in 1..50 {
        new_map = apply_algo(new_map, &algo, min - i, max + i, i % 2 == 1);
    }
    println!("There are {} lights!", count_lights(&new_map));
}

fn count_lights(image: &HashMap<(i32, i32), bool>) -> usize {
    image
        .iter()
        .filter(|(_, light)| **light)
        .count()
}

fn apply_algo(mut lookup: HashMap<(i32, i32), bool>, algo: &Vec<bool>, min: i32, max: i32, default: bool) -> HashMap<(i32, i32), bool> {
    let mut result = HashMap::new();
    // lookup for max +1 and +2 is always default
    for i in 1..3 {
        for x in (min - 2)..=(max + 2) {
            lookup.insert((x, min - i), default);
            lookup.insert((x, max + i), default);
        }
        for y in (min - 2)..=(max + 2) {
            lookup.insert((min - i, y), default);
            lookup.insert((max + i, y), default);
        }
    }
    println!("input image:");
    print(&lookup, min - 2, max + 2);
    for x in (min - 1)..=(max + 1) {
        for y in (min - 1)..=(max + 1) {
            let coord_sum = calc_coord_sum((x, y), &lookup);
            result.insert((x, y), *algo.get(coord_sum as usize).unwrap());
        }
    }
    return result;
}

fn calc_coord_sum((x, y): (i32, i32), lookup: &HashMap<(i32, i32), bool>) -> u32 {
    let lookup_coords = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (0, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1)
    ];
    let res_string: String = lookup_coords
    .iter()
    // we have to calculate what our outer number would be if it's not available yet
    .map(|lookup_coord| {
        let final_coords = (lookup_coord.0 + x, lookup_coord.1 + y);
        if let Some(light) = lookup.get(&final_coords) {
            *light
        } else {
            panic!("We don't have an entry for {:?}!", final_coords);
        }
    })
    .map(|b| if b { "1" } else { "0" })
    .collect();
    u32::from_str_radix(&res_string, 2).unwrap()
}

fn read_input(path: &str) -> (Vec<bool>, HashMap<(i32, i32), bool>) {
    let data = fs::read_to_string(path).unwrap();
    let (algo, image) = data
        .split("\n\n")
        .fold((None, None), |(algo, _), line| {
            if let Some(algo) = algo {
                let image: Vec<Vec<bool>> = line.split("\n").map(|image_line| image_line.chars().map(|c| c == '#').collect()).collect();
                (Some(algo), Some(image))
            } else {
                (Some(line.chars().map(|c| c == '#').collect()), None)
            }
        });
    let map = image.unwrap().iter().enumerate().fold(HashMap::new(), |mut map, (y, line)| {
        map.extend(line.iter().enumerate().map(|(x, light)|((x as i32, y as i32), *light)));
        map
    });
    (algo.unwrap(), map)
}

fn print(map: &HashMap<(i32, i32), bool>, min: i32, max: i32) {
    for y in min..=max {
        for x in min..=max {
            let coords = *map.get(&(x, y)).expect(&format!("Didn't find entry for ({},{})!", x, y));

            print!("{}", if coords { "#" } else { "." });
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use crate::day_20::*;
    #[test]
    fn test_part_1() {
        let (algo, coords) = read_input("./input/day_20.test.txt");
        assert_eq!(algo.len(), 512);
        let lookup_coords =
        [
            ((0, 0), true), ((1, 0), false), ((2, 0), false), ((3, 0), true), ((4, 0), false),
            ((0, 1), true), ((1, 1), false), ((2, 1), false), ((3, 1), false), ((4, 1), false), 
            ((0, 2), true), ((1, 2), true), ((2, 2), false), ((3, 2), false), ((4, 2), true), 
            ((0, 3), false), ((1, 3), false), ((2, 3), true), ((3, 3), false), ((4, 3), false), 
            ((0, 4), false), ((1, 4), false), ((2, 4), true), ((3, 4), true), ((4, 4), true), 
         ]
         .into_iter().collect();
        assert_eq!(coords, lookup_coords);
        let min = 0;
        let max = f32::sqrt(coords.len() as f32).floor() as i32 - 1;
        assert_eq!(calc_coord_sum((2, 2), &coords), 34);
        
        print(&coords, min, max);
        let new_map = apply_algo(coords, &algo, min, max, false);
        assert_eq!(count_lights(&new_map), 24);
        let mut new_map = apply_algo(new_map, &algo, min - 1, max + 1, false);
        println!("result image:");
        print(&new_map, min - 2, max + 2);
        assert_eq!(count_lights(&new_map), 35);
        for i in 2..50 {
            new_map = apply_algo(new_map, &algo, min - i, max + i, false);
        }
        assert_eq!(count_lights(&new_map), 3351);
    }
}