
use std::fs;
use std::collections::{LinkedList, HashMap};

fn read_input(path: &str) -> (LinkedList<char>, HashMap<char, HashMap<char, char>>) {
    let input = fs::read_to_string(path).unwrap();
    let split_input: Vec<&str> = input.trim().split("\n\n").collect();
    let template = LinkedList::from_iter(
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
    (template, instructions)
}

fn polymer_linked_list(iterations: i32, path: &str) -> i32 {
    let (mut template, instructions) = read_input(path);
    let mut new_list = LinkedList::new();
    let mut char_counts = HashMap::new();
    for _ in 0..iterations {
        char_counts = HashMap::new();
        let mut cursor = template.cursor_front();
        let cur = cursor.current().unwrap();
        new_list.push_back(*cur);
        char_counts.entry(*cur).and_modify(|val| *val += 1).or_insert(1);
        while let (Some(cur), Some(next)) = (cursor.current(), cursor.peek_next()) {
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
    char_counts.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1
        - char_counts.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1
}

fn polymer_fast(iterations: i32, path: &str) -> u64 {
    let (template, instructions) = read_input(path);
    let mut patterns = HashMap::new();
    let mut cursor = template.cursor_front();
    let mut char_counts: HashMap<char, u64> = HashMap::new();
    while let (Some(cur), Some(next)) = (cursor.current(), cursor.peek_next()) {
        patterns.entry((cur, next)).and_modify(|f| *f += 1u64).or_insert(1);
        char_counts.entry(*cur).and_modify(|count| *count += 1).or_insert(1);
        cursor.move_next();
    }
    char_counts.entry(*template.back().unwrap()).and_modify(|count| *count += 1).or_insert(1);

    let mut cur_patterns = patterns;
    for _ in 0..iterations {
        let mut new_patterns = HashMap::new();
        for ((from, to), count) in cur_patterns {
            let new_char = instructions.get(from).unwrap().get(to).unwrap();
            new_patterns.entry((from, new_char))
                .and_modify(|f| *f += count)
                .or_insert(count);
            new_patterns.entry((new_char, to))
                .and_modify(|f| *f += count)
                .or_insert(count);
            char_counts
                .entry(*new_char)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
        cur_patterns = new_patterns;
    };
    char_counts.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap().1
        - char_counts.iter().min_by(|(_, x), (_, y)| x.cmp(y)).unwrap().1
}

pub fn part_1() {
    polymer_linked_list(10, "./input/day_14.txt");
}

pub fn part_2() {
    polymer_fast(40, "./input/day_14.txt");
}

#[cfg(test)]
mod tests {
    use crate::day_14::*;
    #[test]
    fn part_1() {
        assert_eq!(polymer_linked_list(10, "./input/day_14.test.txt"), 1588);
    }
    #[test]
    fn part_2() {
        assert_eq!(polymer_fast(40, "./input/day_14.test.txt"), 2188189693529);
    }
}