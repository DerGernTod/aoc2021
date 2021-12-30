use std::{fs, collections::HashMap};

pub fn part_1() {
    let (algo, image) = read_input("./input/day_20.txt");
    let min = (0, 0);
    let max_dir = f32::sqrt(image.len() as f32).floor() as i32;
    let max = (max_dir, max_dir);
    let new_map = apply_algo(&image, &algo, min, max);
    let final_map = apply_algo(&new_map, &algo, (min.0 - 1, min.1 - 1), (max.0 + 1, max.1 + 1));

    println!("There are {} lights!", final_map
        .into_iter()
        .filter(|(_, light)| *light)
        .count()
    );
}

pub fn part_2() {
}

fn apply_algo(lookup: &HashMap<(i32, i32), bool>, algo: &Vec<bool>, min: (i32, i32), max: (i32, i32)) -> HashMap<(i32, i32), bool> {
    let mut result = HashMap::new();
    for x in (min.0 - 1)..(max.0 + 2) {
        for y in (min.1 - 1)..(max.1 + 2) {
            let coord_sum = calc_coord_sum((x, y), lookup);
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
    .map(|lookup_coord| lookup.get(&(lookup_coord.0 + x, lookup_coord.1 + y)))
    .map(|coord_val| coord_val.or(Some(&false)).unwrap())
    .map(|b| if *b { "1" } else { "0" })
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
        assert_eq!(calc_coord_sum((2, 2), &coords), 34);
        let lookup_coords =
        [
            ((-1, -1), false), ((0, -1), true), ((1, -1), true), ((2, -1), false), ((3, -1), true), ((4, -1), true), ((5, -1), false), 
            ((-1, 0), true), ((0, 0), false), ((1, 0), false), ((2, 0), true), ((3, 0), false), ((4, 0), true),((5, 0), false), 
            ((-1, 1), true), ((0, 1), true), ((1, 1), false), ((2, 1), true), ((3, 1), false), ((4, 1), false), ((5, 1), true), 
            ((-1, 2), true), ((0, 2), true), ((1, 2), true), ((2, 2), true), ((3, 2), false), ((4, 2), false), ((5, 2), true), 
            ((-1, 3), false), ((0, 3), true), ((1, 3), false), ((2, 3), false), ((3, 3), true), ((4, 3), true), ((5, 3), false), 
            ((-1, 4), false), ((0, 4), false), ((1, 4), true), ((2, 4), true), ((3, 4), false), ((4, 4), false), ((5, 4), true), 
            ((-1, 5), false), ((0, 5), false), ((1, 5), false), ((2, 5), true), ((3, 5), false), ((4, 5), true), ((5, 5), false), 
         ]
         .into_iter().collect();
         assert_eq!(apply_algo(&coords, &algo, (0, 0), (4, 4)), lookup_coords)
    }
}