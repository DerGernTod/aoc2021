use std::{fs, collections::{HashMap, HashSet}};

pub fn part_1() {
    println!("fastest path is {} scary", find_fastest_path("./input/day_15.txt"));
}
pub fn part_2() {
    println!("fastest path is {} scary", find_in_bigger_path("./input/day_15.txt"));
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Vertex {
    coords: (usize, usize),
    risk: u8,
    dist: Option<u64>
}

fn print_vertices(vertices: &HashMap<(usize, usize), Vertex>) {
    for y in 0..50 {
        println!("");
        for x in 0..50 {
            print!("{}", vertices.get(&(x, y)).unwrap().risk);
        }
    }
    println!("");
}

fn find_in_bigger_path(file: &str) -> u64 {
    let input = fs::read_to_string(file).unwrap();
    let input = input
        .trim()
        .split("\n")
        .map(|line| line
            .split("")
            .filter_map(|c| c.parse::<u8>().ok()));
    
    let vertices: Vec<Vertex> = input
        .enumerate()
        .map(|(y, line)| line
            .enumerate()
            .map(move |(x, risk)| {
                Vertex {
                    coords: (x, y),
                    risk,
                    dist: if x == 0 && y == 0 { Some(0) } else { None }
                }
            }))
        .flatten()
        .collect();
    let size = (vertices.len() as f64).sqrt() as usize;
    let mut vertices: HashMap<(usize, usize), Vertex> = 
        vertices
        .into_iter()
        .fold(HashMap::<(usize, usize), Vertex>::new(), 
        |mut map, vertex| {
            let (vx, vy) = vertex.coords;
            let risk = vertex.risk;
            for x in 0..5u8 {
                for y in 0..5u8 {
                    let new_coords = (
                        vx + size * x as usize,
                        vy + size * y as usize);
                    let new_risk = risk + x + y;
                    map
                        .entry(new_coords)
                        .or_insert(Vertex {
                            coords: new_coords,
                            dist: if new_coords.0 == 0 && new_coords.1 == 0 { Some(0) } else { None },
                            risk: if new_risk > 9 { new_risk % 9 } else { new_risk }
                        });
                }
            }
            map
        });
    print_vertices(&vertices);
    let total_size = size * size * 25;
    let mut vertices_with_dist = HashMap::new();
    vertices_with_dist.insert((0, 0), 0);
    let mut spt_set = HashMap::new();
    while spt_set.len() < total_size {
        println!("{}/{}", spt_set.len(), total_size);
        // find vertex with Some(dist) with lowest dist
        let vertex = vertices_with_dist
            .iter()
            .min_by(|a, b| a.1.cmp(b.1));
        let vertex = match vertex {
            Some(((x, y), _)) => vertices.get(&(*x, *y)).unwrap(),
            None => panic!("Didn't find a vertex that isn't part of spt_set and has a dist! set is {:?}",
                spt_set.len())
        };
        let vertex_dist = vertex.dist.unwrap();
        let v_coords = vertex.coords;
        let (x , y) = v_coords;
        vertices_with_dist.remove(&v_coords);
        let v = vertices.remove(&v_coords).unwrap();
        spt_set.insert(v_coords, v);
        [
            (Some(x + 1), Some(y)),
            (usize::checked_sub(x, 1), Some(y)),
            (Some(x), Some(y + 1)),
            (Some(x), usize::checked_sub(y, 1))]
            .into_iter()
            .filter(|(x, y)| x.is_some() && y.is_some())
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .for_each(|(x, y)| {
                let v = vertices
                    .get_mut(&(x, y));
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
                vertices_with_dist.insert((x, y), new_dist);
            });
        if v_coords.0 == size * 5 - 1 && v_coords.1 == size * 5 - 1 {
            break;
        }
    }
    spt_set.get(&(size * 5 - 1, size * 5 - 1)).unwrap().dist.unwrap()
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
                println!("Adjacent to {}, {}: {:?}", v_coords.0, v_coords.1, v);
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

    #[test]
    fn part_2() {
        assert_eq!(find_in_bigger_path("./input/day_15.test.txt"), 315);
    }
}
