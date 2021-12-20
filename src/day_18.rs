use std::{fs, rc::Rc};

#[derive(PartialEq, Eq, Debug)]
struct NumberInfo {
    value: u32,
    depth: u32,
    parent: Option<Rc<PairInfo>>
}

#[derive(PartialEq, Eq, Debug)]
struct PairInfo {
    left: Rc<NumberEntry>,
    right: Rc<NumberEntry>,
    depth: u32,
    parent: Option<Rc<PairInfo>>
}

impl NumberInfo {
    fn new(value: u32, depth: u32) -> NumberInfo {
        NumberInfo {
            value,
            depth,
            parent: None
        }
    }
    fn set_parent(&mut self, parent: Rc<PairInfo>) {
        self.parent = Some(parent);
    }
}

impl PairInfo {
    fn new(left: Rc<NumberEntry>, right: Rc<NumberEntry>, depth: u32) -> PairInfo {
        PairInfo {
            left,
            right,
            depth,
            parent: None
        }
    }
    fn set_parent(&mut self, parent: Rc<PairInfo>) {
        self.parent = Some(parent);
    }
}

#[derive(PartialEq, Eq, Debug)]
enum NumberEntry {
    Literal(Rc<NumberInfo>),
    Pair(Rc<PairInfo>),
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
    let result_pair = create_pair(0, &mut cursor, str.get(0).unwrap());
    println!("parsed results: {:?}", result_pair);
}

fn create_pair(depth: u32, cursor: &mut usize, cur_operation: &Vec<char>) -> Rc<PairInfo> {
    *cursor += 1;
    let c = cur_operation.get(*cursor).unwrap();
    let left = Rc::new(match c {
        '[' => NumberEntry::Pair(create_pair(depth + 1, cursor, cur_operation)),
        '0'..='9' => NumberEntry::Literal(Rc::new(NumberInfo::new(c.to_digit(10).unwrap(), depth))),
        x => panic!("We got something other than '[' or 0-9 after a '[' at index {}: {:?}", *cursor, x)
    });
    // we must be at ] or number now, so skip this and a ,
    *cursor += 2;
    let c = cur_operation.get(*cursor).unwrap();
    let right = Rc::new(match c {
        '[' => NumberEntry::Pair(create_pair(depth + 1, cursor, cur_operation)),
        '0'..='9' => NumberEntry::Literal(Rc::new(NumberInfo::new(c.to_digit(10).unwrap(), depth))),
        x => panic!("We got something other than '[' or 0-9 after a ',' at index {}: {:?}", *cursor, x)
    });
    // we must be at a ']' now
    *cursor += 1;
    let pair = PairInfo::new(Rc::clone(&left), Rc::clone(&right), depth + 1);
    let pair_rc = Rc::new(pair);
    match Rc::clone(&left).as_ref() {
        NumberEntry::Literal(mut x) => x.set_parent(Rc::clone(&pair_rc)),
        NumberEntry::Pair(mut x) => x.set_parent(Rc::clone(&pair_rc)),
        _ => ()
    };
    match right.as_ref() {
        NumberEntry::Literal(mut x) => x.set_parent(Rc::clone(&pair_rc)),
        NumberEntry::Pair(mut x) => x.set_parent(Rc::clone(&pair_rc)),
        _ => ()
    };
    pair_rc
}

#[cfg(test)]
mod tests {
    use crate::day_18::*;
    #[test]
    fn test_part_1() {
        add_fishnumbers_from_path("./input/day_18.test.txt");
    }
}