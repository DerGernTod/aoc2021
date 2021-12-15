use std::{fs, collections::{HashMap, HashSet}, cell::RefCell, rc::Rc, slice::SliceIndex};

pub fn part_1() {
    println!("fastest path is {} scary", find_fastest_path("./input/day_15.txt"));
}
pub fn part_2() {}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Vertex {
    coords: (usize, usize),
    risk: u8,
    dist: Option<u64>
}

fn find_fastest_path(file: &str) -> u64 {
    let input = fs::read_to_string(file).unwrap();
    let input = input
        .trim()
        .split("\n")
        .map(|line| line
            .split("")
            .filter_map(|c| c.parse::<u8>().ok()));
    
    let mut graph: Vec<Vec<Vertex>> = input
        .enumerate()
        .map(|(y, line)| line
            .enumerate()
            .map(|(x, risk)| Vertex {
                coords: (x, y),
                risk,
                dist: if x == 0 && y == 0 { Some(0) } else { None }
            })
            .collect())
        .collect();
    
    let total_size = graph.len() * graph[0].len();
    let mut spt_set = HashSet::new();
    while spt_set.len() < total_size {
        // find vertex with Some(dist) with lowest dist
        let vertex = graph
            .iter()
            .flatten()
            .filter(|v| !spt_set.contains(&(v.coords.0, v.coords.1)))
            .filter(|v| v.dist.is_some())
            .min_by(|v1, v2| v1.dist.unwrap().cmp(&v2.dist.unwrap()));
        let vertex = match vertex {
            Some(v) => v,
            None => panic!("Didn't find a vertex that isn't part of spt_set and has a dist!")
        };
        let vertex_dist = vertex.dist.unwrap();
        let v_coords = vertex.coords;
        let (x , y) = v_coords;
        spt_set.insert((x, y));
        let adjacent_coords = [
            (Some(x + 1), Some(y)),
            (usize::checked_sub(x, 1), Some(y)),
            (Some(x), Some(y + 1)),
            (Some(x), usize::checked_sub(y, 1))];
        adjacent_coords
            .iter()
            .filter(|(x, y)| x.is_some() && y.is_some())
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .filter(|(x, y)| !spt_set.contains(&(*x, *y)))
            .for_each(|(x, y)| {
                let v = graph
                    .get_mut(y)
                    .and_then(|line| line.get_mut(x));
                let v = match v {
                    None => return,
                    Some(v) => v
                };
                let sum = vertex_dist + v.risk as u64;
                let new_dist = match v.dist {
                    None => sum,
                    Some(dist) if sum < dist => sum,
                    Some(dist) => dist
                };
                v.dist = Some(new_dist);
            });
    }
    graph.last().unwrap().last().unwrap().dist.unwrap()
}


#[cfg(test)]
mod tests {
    use crate::day_15::*;

    #[test]
    fn part_1() {
        assert_eq!(find_fastest_path("./input/day_15.test.txt"), 40);
    }
}
