
use std::fs;
use std::collections::{LinkedList, HashMap};
use std::hash::Hash;

fn polymer(iterations: i32) {
    let input = fs::read_to_string("./input/day_14.txt").unwrap();
    let split_input: Vec<&str> = input.trim().split("\n\n").collect();
    let mut template = LinkedList::from_iter(
        split_input[0].chars());
    let instructions: HashMap<char, HashMap<char, char>> = split_input[1]
        .trim()
        .split("\n")
        .map(|str| str
            .split(" -> ")
            .map(|str| str.chars())
            .flatten()
            .collect::<Vec<char>>())
        .fold(HashMap::new(), | mut acc, chars | {
            let entry = acc.entry(chars[0]);
            entry
                .or_insert(HashMap::new())
                .entry(chars[1]).or_insert(chars[2]);
            
            return acc;
        });
    let mut new_list = LinkedList::new();
    let mut char_counts = HashMap::new();
    for _ in 0..iterations {
        char_counts = HashMap::new();
        let mut cursor = template.cursor_front();
        let cur = cursor.current().unwrap();
        new_list.push_back(*cur);
        char_counts.entry(*cur).and_modify(|val| *val += 1).or_insert(1);
        while let Some(next) = cursor.peek_next() {
            let cur = cursor.current().unwrap();
            let insert_char = instructions.get(cur).unwrap().get(next).unwrap();
            new_list.push_back(*insert_char);
            char_counts.entry(*insert_char).and_modify(|val| *val += 1).or_insert(1);
            new_list.push_back(*next);
            char_counts.entry(*next).and_modify(|val| *val += 1).or_insert(1);
            cursor.move_next();
        }
        template = new_list;
        new_list = LinkedList::new();
    }
    println!("min: {:?}, max: {:?}",
        char_counts.iter().min_by(|a, b| a.1.cmp(b.1)),
        char_counts.iter().max_by(|a, b| a.1.cmp(b.1)));
}

pub fn part_1() {
    polymer(10);
}

pub fn part_2() {
    polymer(10);
}