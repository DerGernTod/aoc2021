use std::{fs, collections::HashMap};

pub fn part_1() {
    let (algo, image) = read_input("./input/day_20.txt");
    let min = 0;
    let max = f32::sqrt(image.len() as f32).floor() as i32;
    let new_map = apply_algo(image, &algo, min, max, false);
    let new_map = apply_algo(new_map, &algo, min - 3, max + 3, algo[255]);

    println!("There are {} lights!", count_lights(&new_map));
}

pub fn part_2() {
}

fn count_lights(image: &HashMap<(i32, i32), bool>) -> usize {
    image
        .iter()
        .filter(|(_, light)| **light)
        .count()
}

fn apply_algo(mut lookup: HashMap<(i32, i32), bool>, algo: &Vec<bool>, min: i32, max: i32, default: bool) -> HashMap<(i32, i32), bool> {
    let mut result = HashMap::new();
    let min = min - 3;
    let max = max + 3;
    for x in min..max {
        lookup.insert((x, min), default);
        lookup.insert((x, max), default);
        lookup.insert((x, min + 1), default);
        lookup.insert((x, max - 1), default);
    }
    for y in min..max {
        lookup.insert((min, y), default);
        lookup.insert((max, y), default);
        lookup.insert((min + 1, y), default);
        lookup.insert((max - 1, y), default);
    }
    for x in min..max {
        for y in min..max {
            let coord_sum = calc_coord_sum((x, y), &lookup, &algo, default, min, max);
            result.insert((x, y), *algo.get(coord_sum as usize).unwrap());
        }
    }
    return result;
}

fn bool_vec_to_num(v: Vec<bool>) -> usize {
    let res_string: String = v.into_iter().map(|b| if b { "1" } else { "0" }).collect();
    usize::from_str_radix(&res_string, 2).unwrap()
}

fn calc_border(final_coords: (i32, i32), lookup: &HashMap<(i32, i32), bool>, algo: &Vec<bool>, default: bool, min: i32, max: i32) -> bool {
    match final_coords {
        // upper left corner
        (x, y) if x - 1 == min && y - 1 == min => algo[bool_vec_to_num(vec![
            default, default, default,
            default, default, default,
            default, default, *lookup.get(&(x + 1, y + 1)).unwrap(),
        ])],
        // upper right corner
        (x, y) if x + 1 == max && y - 1 == min => algo[bool_vec_to_num(vec![
            default, default, default,
            default, default, default,
            *lookup.get(&(x - 1, y + 1)).unwrap(), default, default,
        ])],
        // lower left corner
        (x, y) if x - 1 == min && y + 1 == max => algo[bool_vec_to_num(vec![
            default, default, *lookup.get(&(x + 1, y - 1)).unwrap(), 
            default, default, default,
            default, default, default,
        ])],
        // lower right corner
        (x, y) if x + 1 == max && y + 1 == max => algo[bool_vec_to_num(vec![
            *lookup.get(&(x - 1, y - 1)).unwrap(), default, default,
            default, default, default,
            default, default, default,
        ])],
        // left edge upper
        (x, y) if x - 1 == min && y == min => algo[bool_vec_to_num(vec![
            default, default, default, 
            default, default, *lookup.get(&(x + 1, y)).unwrap(), 
            default, default, *lookup.get(&(x + 1, y + 1)).unwrap(), 
        ])],
        // left edge lower
        (x, y) if x - 1 == min && y == max => algo[bool_vec_to_num(vec![
            default, default, *lookup.get(&(x + 1, y - 1)).unwrap(), 
            default, default, *lookup.get(&(x + 1, y)).unwrap(), 
            default, default, default, 
        ])],
        // right edge upper
        (x, y) if x + 1 == max && y == min => algo[bool_vec_to_num(vec![
            default, default, default, 
            *lookup.get(&(x - 1, y)).unwrap(), default, default, 
            *lookup.get(&(x - 1, y + 1)).unwrap(), default, default,
        ])],
        // right edge lower
        (x, y) if x + 1 == max => algo[bool_vec_to_num(vec![
            *lookup.get(&(x - 1, y - 1)).unwrap(), default, default, 
            *lookup.get(&(x - 1, y)).unwrap(), default, default, 
            default, default, default,
        ])],
        // top edge left
        (x, y) if y - 1 == min && x == min => algo[bool_vec_to_num(vec![
            default, default, default,
            default, default, default,
            default, *lookup.get(&(x, y + 1)).unwrap(), *lookup.get(&(x + 1, y + 1)).unwrap(),
        ])],
        // top edge right
        (x, y) if y - 1 == min && x == max => algo[bool_vec_to_num(vec![
            default, default, default,
            default, default, default,
            *lookup.get(&(x - 1, y + 1)).unwrap(), *lookup.get(&(x, y + 1)).unwrap(), default,
        ])],
        // bottom edge left
        (x, y) if y + 1 == max && x == min => algo[bool_vec_to_num(vec![
            default, *lookup.get(&(x, y - 1)).unwrap(), *lookup.get(&(x + 1, y - 1)).unwrap(),
            default, default, default,
            default, default, default,
        ])],
        // bottom edge right
        (x, y) if y + 1 == max && x == max => algo[bool_vec_to_num(vec![
            *lookup.get(&(x - 1, y - 1)).unwrap(), *lookup.get(&(x, y - 1)).unwrap(), default,
            default, default, default,
            default, default, default,
        ])],
        // left edge
        (x, y) if x - 1 == min => algo[bool_vec_to_num(vec![
            default, default, *lookup.get(&(x + 1, y - 1)).unwrap(), 
            default, default, *lookup.get(&(x + 1, y)).unwrap(), 
            default, default, *lookup.get(&(x + 1, y + 1)).unwrap(), 
        ])],
        // right edge
        (x, y) if x + 1 == max => algo[bool_vec_to_num(vec![
            *lookup.get(&(x - 1, y - 1)).unwrap(), default, default, 
            *lookup.get(&(x - 1, y)).unwrap(), default, default, 
            *lookup.get(&(x - 1, y + 1)).unwrap(), default, default,
        ])],
        // top edge
        (x, y) if y - 1 == min => algo[bool_vec_to_num(vec![
            default, default, default,
            default, default, default,
            *lookup.get(&(x - 1, y + 1)).unwrap(), *lookup.get(&(x, y + 1)).unwrap(), *lookup.get(&(x + 1, y + 1)).unwrap(),
        ])],
        // bottom edge
        (x, y) if y + 1 == max => algo[bool_vec_to_num(vec![
            *lookup.get(&(x - 1, y - 1)).unwrap(), *lookup.get(&(x, y - 1)).unwrap(), *lookup.get(&(x + 1, y - 1)).unwrap(),
            default, default, default,
            default, default, default,
        ])],
        _ => default
    }
}


fn calc_coord_sum((x, y): (i32, i32), lookup: &HashMap<(i32, i32), bool>, algo: &Vec<bool>, default: bool, min: i32, max: i32) -> u32 {
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
            calc_border(final_coords, lookup, algo, default, min, max)
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
        let max = f32::sqrt(coords.len() as f32).floor() as i32;
        assert_eq!(calc_coord_sum((2, 2), &coords, &algo, false, min, max), 34);
        // let lookup_coords =
        // [
        //     ((-1, -1), false), ((0, -1), true), ((1, -1), true), ((2, -1), false), ((3, -1), true), ((4, -1), true), ((5, -1), false), 
        //     ((-1, 0), true), ((0, 0), false), ((1, 0), false), ((2, 0), true), ((3, 0), false), ((4, 0), true),((5, 0), false), 
        //     ((-1, 1), true), ((0, 1), true), ((1, 1), false), ((2, 1), true), ((3, 1), false), ((4, 1), false), ((5, 1), true), 
        //     ((-1, 2), true), ((0, 2), true), ((1, 2), true), ((2, 2), true), ((3, 2), false), ((4, 2), false), ((5, 2), true), 
        //     ((-1, 3), false), ((0, 3), true), ((1, 3), false), ((2, 3), false), ((3, 3), true), ((4, 3), true), ((5, 3), false), 
        //     ((-1, 4), false), ((0, 4), false), ((1, 4), true), ((2, 4), true), ((3, 4), false), ((4, 4), false), ((5, 4), true), 
        //     ((-1, 5), false), ((0, 5), false), ((1, 5), false), ((2, 5), true), ((3, 5), false), ((4, 5), true), ((5, 5), false), 
        // ]
        // .into_iter().collect();
        let new_map = apply_algo(coords, &algo, min, max, false);
        // assert_eq!(new_map, lookup_coords);
        assert_eq!(count_lights(&new_map), 24);
        let new_map = apply_algo(new_map, &algo, min - 3, max + 3, false);
        assert_eq!(count_lights(&new_map), 35);
    }
}