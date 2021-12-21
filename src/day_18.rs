use std::{fs};
use self::number_entry::*;
mod number_entry;

pub fn part_1() {
    let mut ops = create_fishnumbers_from_path("./input/day_18.txt");
    let mut first_num = ops.remove(0);
    let first_op = ops.remove(0);
    add(&mut first_num, first_op);
}

pub fn part_2() {

}

fn create_fishnumbers_from_path(path: &str) -> Vec<Vec<NumberEntry>> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split("\n")
        .map(|str| str.chars().collect())
        .map(|char_vec| {
            let mut cursor = 0;
            let mut lookup = vec![];
            create_pair(0, &mut cursor, &char_vec, &mut lookup, None);
            lookup
        }).collect()
}

fn create_literal(depth: usize, value: usize, lookup: &mut Vec<NumberEntry>, parent_id: usize) -> usize {
    let id = lookup.len();
    let entry = NumberEntry::Literal(NumberInfo::new(id, value, depth, Some(parent_id)));
    lookup.push(entry);
    id
}

fn create_pair(depth: usize, cursor: &mut usize, cur_operation: &Vec<char>, lookup: &mut Vec<NumberEntry>, parent: Option<usize>) -> usize {
    let pair_id = lookup.len();
    lookup.push(NumberEntry::None);
    *cursor += 1;
    let c = cur_operation.get(*cursor).unwrap();
    let left = match c {
        '[' => create_pair(depth + 1, cursor, cur_operation, lookup, Some(pair_id)),
        '0'..='9' => create_literal(depth + 1, c.to_digit(10).unwrap() as usize, lookup, pair_id),
        x => panic!("We got something other than '[' or 0-9 after a '[' at index {}: {:?}", *cursor, x)
    };
    // we must be at ] or number now, so skip this and a ,
    *cursor += 2;
    let c = cur_operation.get(*cursor).unwrap();
    let right = match c {
        '[' => create_pair(depth + 1, cursor, cur_operation, lookup, Some(pair_id)),
        '0'..='9' => create_literal(depth + 1, c.to_digit(10).unwrap() as usize, lookup, pair_id),
        x => panic!("We got something other than '[' or 0-9 after a ',' at index {}: {:?}", *cursor, x)
    };
    // we must be at a ']' now
    *cursor += 1;
    
    let new_entry = NumberEntry::Pair(PairInfo::new(pair_id, left, right, depth, parent));
    lookup[pair_id] = new_entry;
    pair_id
}

fn increment_depth(entry: usize, lookup: &mut Vec<NumberEntry>) -> bool {
    let entry = lookup.get_mut(entry).unwrap();
    match entry {
        NumberEntry::Literal(l) => {
            l.depth += 1;
            false
        },
        NumberEntry::Pair(p) => {
            p.depth += 1;
            p.depth >= 4
        },
        _ => false
    }
}

fn update_parent(entry: &mut NumberEntry, new_parent: Option<usize>) {
    match entry {
        NumberEntry::Literal(l) => {
            l.parent = new_parent;
        },
        NumberEntry::Pair(p) => {
            p.parent = new_parent;
        },
        _ => ()
    };
}

fn find_right_most_in_children_literal_id(entry_id: usize, lookup: &mut Vec<NumberEntry>) -> Option<usize> {
    let entry = lookup.get(entry_id).unwrap();
    match entry {
        NumberEntry::Literal(_) => Some(entry_id),
        NumberEntry::Pair(p) => {
            let right_id = p.right;
            let right = lookup.get(right_id).unwrap();
            match right {
                NumberEntry::Literal(_) => Some(right_id),
                NumberEntry::Pair(_) => find_right_most_in_children_literal_id(right_id, lookup),
                NumberEntry::None => None
            }
        },
        NumberEntry::None => None
    }
}

fn find_left_most_in_children_literal_id(entry_id: usize, lookup: &mut Vec<NumberEntry>) -> Option<usize> {
    let entry = lookup.get(entry_id).unwrap();
    match entry {
        NumberEntry::Literal(_) => Some(entry_id),
        NumberEntry::Pair(p) => {
            let left_id = p.left;
            let left = lookup.get(left_id).unwrap();
            match left {
                NumberEntry::Literal(_) => Some(left_id),
                NumberEntry::Pair(_) => find_left_most_in_children_literal_id(left_id, lookup),
                NumberEntry::None => None
            }
        },
        NumberEntry::None => None
    }
}

fn find_right_most_in_parent_literal_id(entry_id: usize, lookup: &mut Vec<NumberEntry>) -> Option<usize> {
    let entry = lookup.get(entry_id).unwrap();
    match entry {
        NumberEntry::Literal(_) => Some(entry_id),
        NumberEntry::Pair(_) => {
            let parent = entry.get_parent();
            if let Some(parent_id) = parent {
                let parent = lookup.get(parent_id).unwrap();
                let right_id = parent.right();
                if right_id == entry_id {
                    // fetch the left most of the right of us
                    find_right_most_in_parent_literal_id(parent_id, lookup)
                } else {
                    // fetch the right most of the left of us
                    find_left_most_in_children_literal_id(right_id, lookup)
                }
            } else {
                None
            }
        },
        NumberEntry::None => None
    }
}

fn find_left_most_in_parent_literal_id(entry_id: usize, lookup: &mut Vec<NumberEntry>) -> Option<usize> {
    let entry = lookup.get(entry_id).unwrap();
    match entry {
        NumberEntry::Literal(_) => Some(entry_id),
        NumberEntry::Pair(_) => {
            let parent = entry.get_parent();
            if let Some(parent_id) = parent {
                let parent = lookup.get(parent_id).unwrap();
                let left_id = parent.left();
                if left_id == entry_id {
                    // fetch the left most of the right of us
                    find_left_most_in_parent_literal_id(parent_id, lookup)
                } else {
                    // fetch the right most of the left of us
                    find_right_most_in_children_literal_id(left_id, lookup)
                }
            } else {
                None
            }
        },
        NumberEntry::None => None
    }
}

fn explode(entry_id: usize, lookup: &mut Vec<NumberEntry>) {
    let entry = lookup.get_mut(entry_id).unwrap();
    let left_id = entry.left();
    let right_id = entry.right();
    println!("Exploding id {} with {} and {}", entry_id, left_id, right_id);
    let parent_id = entry.get_parent();
    let depth = entry.get_depth();
    let left_val = lookup.get(left_id).unwrap().value();
    let left_literal_id = find_left_most_in_parent_literal_id(entry_id, lookup);
    if let Some(left_literal_id) = left_literal_id {
        let left_literal = lookup.get_mut(left_literal_id).unwrap();
        if let NumberEntry::Literal(l) = left_literal {
            l.value += left_val;
        } else {
            panic!("Left most ist not a literal, but instead {:?}!", left_literal);
        }
    }
    
    let right_val = lookup.get(right_id).unwrap().value();
    let right_literal_id = find_right_most_in_parent_literal_id(entry_id, lookup);
    if let Some(right_literal_id) = right_literal_id {
        let right_literal = lookup.get_mut(right_literal_id).unwrap();
        if let NumberEntry::Literal(l) = right_literal {
            l.value += right_val;
        } else {
            panic!("Right most ist not a literal, but instead {:?}!", right_literal);
        }
    }
    lookup[left_id] = NumberEntry::None;
    lookup[right_id] = NumberEntry::None;
    lookup[entry_id] = NumberEntry::Literal(NumberInfo::new(entry_id, 0, depth, parent_id));
}

fn div_up(a: usize, b: usize) -> usize {
    // We *know* that the hint is exact, this is thus precisely the amount of chunks of length `b` each
    (0..a).step_by(b).size_hint().0
}

fn split(split_id: usize, lookup: &mut Vec<NumberEntry>) {
    let split = lookup.get(split_id).unwrap();
    let split_value = split.value();
    let split_depth = split.get_depth();
    let parent_id = split.get_parent();
    let new_left_id = lookup.len();
    let new_right_id = lookup.len() + 1;
    let new_left = NumberEntry::Literal(NumberInfo::new(
        new_left_id, 
        split_value / 2, 
        split_depth + 1, 
        Some(split_id)
    ));
    let new_right = NumberEntry::Literal(NumberInfo::new(
        new_right_id, 
        div_up(split_value, 2), 
        split_depth + 1, 
        Some(split_id)
    ));
    lookup.push(new_left);
    lookup.push(new_right);
    println!("Splitting id {} into {} and {}", split_id, new_left_id, new_right_id);
    lookup[split_id] = NumberEntry::Pair(PairInfo::new(
        split_id,
        new_left_id,
        new_right_id,
        split_depth,
        parent_id
    ));
}

fn reindex(offset: usize, lookup_left: &mut Vec<NumberEntry>, lookup_right: Vec<NumberEntry>) {
    lookup_right
        .into_iter()
        .for_each(|mut num| {
            num.set_offset(offset);
            lookup_left.push(num);
    });
}

fn check_explosions(lookup: &Vec<NumberEntry>) -> Option<usize> {
    lookup
        .iter()
        .filter_map(|num| {
            if let NumberEntry::Pair(p) = num {
                if p.depth >= 4 {
                    return Some(p.id);
                }
            }
            None
        })
        .map(|id| (id, calc_left_score(id, lookup, 1)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(id, _)| id)
}

fn check_splits(lookup: &Vec<NumberEntry>) -> Option<usize> {
    lookup
        .iter()
        .filter_map(|num| {
            if let NumberEntry::Literal(num) = num {
                if num.value > 9 {
                    return Some(num.id);
                }
            };
            None
        })
        .map(|id| (id, calc_left_score(id, lookup, 1)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(id, _)| id)
}

fn calc_left_score(id: usize, lookup: &Vec<NumberEntry>, dist: usize) -> usize {
    let num = lookup.get(id).unwrap();
    let parent = num.get_parent();
    let mut score = 0;
    if let Some(parent) = parent {

        let parent_num = lookup.get(parent).unwrap();
        if parent_num.left() == id {
            score += 1 * dist;
        }
        score += calc_left_score(parent, lookup, dist + 1);
    }
    score
}

fn add(lookup_left: &mut Vec<NumberEntry>, lookup_right: Vec<NumberEntry>) {
    let offset = lookup_left.len();
    reindex(offset, lookup_left, lookup_right);
    let new_id = lookup_left.len();
    update_parent(lookup_left.get_mut(0).unwrap(), Some(new_id));
    update_parent(lookup_left.get_mut(offset).unwrap(), Some(new_id));
    lookup_left.push(NumberEntry::Pair(PairInfo::new(new_id, 0, offset, 0, None)));
    for num in 0..lookup_left.len() - 1 {
        increment_depth(num, lookup_left);
    }
    loop {
        if let Some(ex_id) = check_explosions(lookup_left) {
            explode(ex_id, lookup_left);
        } else if let Some(spl_id) = check_splits(lookup_left) {
            split(spl_id, lookup_left);
        } else {
            break;
        }
    }
}

fn find_root(num: &Vec<NumberEntry>) -> usize {
    num.iter().find_map(|num| {
        if let NumberEntry::Pair(p) = num {
            if num.get_parent() == None {
                return Some(num.get_id());
            }
        };
        None
    }).unwrap()
}

fn magnitude(num_id: usize, lookup: &Vec<NumberEntry>) -> usize {
    let num = lookup.get(num_id).unwrap();
    match num {
        NumberEntry::Literal(l) => l.value,
        NumberEntry::Pair(num) => {
            magnitude(num.left, lookup) * 3
            + magnitude(num.right, lookup) * 2
        },
        NumberEntry::None => 0
    }
}

fn print(entry: usize, lookup: &Vec<NumberEntry>) {
    let entry = lookup.get(entry).unwrap();
    match entry {
        NumberEntry::Literal(l) => print!("{}", l.value),
        NumberEntry::Pair(p) => {
            print!("[");
            print(p.left, lookup);
            print!(",");
            print(p.right, lookup);
            print!("]");
        },
        NumberEntry::None => ()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_18::*;
    #[test]
    fn test_part_1() {
        let mut ops = create_fishnumbers_from_path("./input/day_18.test.txt");
        let mut first_num = ops.remove(0);
        while ops.len() > 0 {
            let first_op = ops.remove(0);
            add(&mut first_num, first_op);
            print(find_root(&first_num), &first_num);
        }
        assert_eq!(magnitude(find_root(&first_num), &first_num), 3488);
    }
    #[test]
    fn test_part_1_simple() {
        let mut ops = create_fishnumbers_from_path("./input/day_18.test.simple.txt");
        let mut first_num = ops.remove(0);
        while ops.len() > 0 {
            let first_op = ops.remove(0);
            add(&mut first_num, first_op);
        }
        assert_eq!(magnitude(find_root(&first_num), &first_num), 1384);
    }
}