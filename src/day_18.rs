use std::{fs, rc::Rc, collections::HashMap};

#[derive(PartialEq, Eq, Debug)]
struct NumberInfo {
    value: u32,
    depth: u32,
    parent: usize
}

#[derive(PartialEq, Eq, Debug)]
struct PairInfo {
    left: usize,
    right: usize,
    depth: u32,
    parent: usize
}

impl NumberInfo {
    fn new(value: u32, depth: u32, parent_id: usize) -> NumberInfo {
        NumberInfo {
            value,
            depth,
            parent: parent_id
        }
    }
    fn set_parent(&mut self, parent: usize) {
        self.parent = parent;
    }
}

impl PairInfo {
    fn new(left: usize, right: usize, depth: u32, parent_id: usize) -> PairInfo {
        PairInfo {
            left,
            right,
            depth,
            parent: parent_id
        }
    }
    fn set_parent(&mut self, parent: usize) {
        self.parent = parent;
    }
}

#[derive(PartialEq, Eq, Debug)]
enum NumberEntry {
    Literal(NumberInfo),
    Pair(PairInfo),
    None
}

pub fn part_1() {
    add_fishnumbers_from_path("./input/day_18.txt");
}

pub fn part_2() {

}

fn add_fishnumbers_from_path(path: &str)  {
    let str: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|str| str.chars().into_iter().collect())
        .collect();
    let mut cursor = 0;
    let mut lookup = HashMap::new();
    create_pair(0, &mut cursor, str.get(0).unwrap(), &mut lookup, 0);
    println!("parsed results: {:?}", lookup);
}

fn create_literal(depth: u32, cursor: &mut usize, value: u32, lookup: &mut HashMap<usize, NumberEntry>, parent_id: usize) -> usize {
    let entry = NumberEntry::Literal(NumberInfo::new(value, depth, parent_id));
    lookup.entry(*cursor).or_insert(entry);
    *cursor
}

fn create_pair(depth: u32, cursor: &mut usize, cur_operation: &Vec<char>, lookup: &mut HashMap<usize, NumberEntry>, parent_id: usize) -> usize {
    let pair_id = *cursor;
    *cursor += 1;
    let c = cur_operation.get(*cursor).unwrap();
    let left = match c {
        '[' => create_pair(depth + 1, cursor, cur_operation, lookup, pair_id),
        '0'..='9' => create_literal(depth + 1, cursor, c.to_digit(10).unwrap(), lookup, pair_id),
        x => panic!("We got something other than '[' or 0-9 after a '[' at index {}: {:?}", *cursor, x)
    };
    // we must be at ] or number now, so skip this and a ,
    *cursor += 2;
    let c = cur_operation.get(*cursor).unwrap();
    let right = match c {
        '[' => create_pair(depth + 1, cursor, cur_operation, lookup, pair_id),
        '0'..='9' => create_literal(depth + 1, cursor, c.to_digit(10).unwrap(), lookup, pair_id),
        x => panic!("We got something other than '[' or 0-9 after a ',' at index {}: {:?}", *cursor, x)
    };
    // we must be at a ']' now
    *cursor += 1;
    lookup.entry(pair_id).or_insert(NumberEntry::Pair(PairInfo::new(left, right, depth + 1, parent_id)));
    pair_id
}

#[cfg(test)]
mod tests {
    use crate::day_18::*;
    #[test]
    fn test_part_1() {
        add_fishnumbers_from_path("./input/day_18.test.txt");
    }
}