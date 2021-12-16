use std::{fs, collections::HashSet};
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Dot(i32, i32);
impl Dot {
    fn from(str: &str) -> Dot {
        let from = String::from(str);
        if let Some(i) = from.find(',') {
            return Dot(
                i32::from_str_radix(&from[..i], 10).unwrap(),
                i32::from_str_radix(&from[i+1..], 10).unwrap()
            );
        }
        panic!("Separator ',' not found in string {}!", str);
    }
}
#[derive(Debug)]
struct Instruction(i32, i32);
impl Instruction {
    fn from(entry: Vec<&str>) -> Instruction {
        if let Ok(num) = i32::from_str_radix(entry[1], 10) {
           return Instruction((entry[0].chars().nth(11).unwrap() == 'y') as i32, num);
        }
        panic!("Invalid entries for instruction {:?}!", entry);
    }
}

pub fn part_1() {
    let (instructions, dots) = read_input();
    fold_first(instructions, dots);
}

pub fn part_2() {
    let (instructions, dots) = read_input();
    fold(instructions, dots);
}

fn read_input() -> (Vec<Instruction>, Vec<Dot>) {
    let contents =
        fs::read_to_string("./input/day_13.txt")
        .expect("Something went wrong reading the file");
    let splits = contents.split("\n\n").collect::<Vec<&str>>();
    let instructions = splits[1]
        .trim()
        .split("\n")
        .map(|instr| Instruction::from(instr.split("=").collect::<Vec<&str>>()))
        .collect();
    let result_dots = splits[0]
        .trim()
        .split("\n")
        .map(|dot| Dot::from(dot))
        .collect();
    (instructions, result_dots)
}

fn fold_first(instructions: Vec<Instruction>, dots: Vec<Dot>) {
    let axis = instructions[0].0;
    let coord = instructions[0].1;
    let result_dots = dots
        .iter()
        .fold(HashSet::<Dot>::from_iter(dots.iter().cloned()), |mut acc, cur| {
            if axis == 0 && cur.0 >= coord {
                acc.remove(&cur);
                acc.insert(Dot(coord * 2 - cur.0, cur.1));
            } else if axis == 1 && cur.1 >= coord {
                acc.remove(&cur);
                acc.insert(Dot(cur.0, coord * 2 - cur.1));
            }
            acc
        });
    println!("Result number dots: {}", result_dots.len())
}

fn fold(instructions: Vec<Instruction>, dots: Vec<Dot>) {
    let result_hash_set = instructions
        .iter()
        .fold(HashSet::<Dot>::from_iter(dots.iter().cloned()), |acc, Instruction(axis, coord)| acc
            .iter()
            .fold(acc.clone(), |mut acc, cur| {
                if *axis == 0 && cur.0 >= *coord {
                    acc.remove(&cur);
                    acc.insert(Dot(coord * 2 - cur.0, cur.1));
                } else if *axis == 1 && cur.1 >= *coord {
                    acc.remove(&cur);
                    acc.insert(Dot(cur.0, coord * 2 - cur.1));
                }
                acc
            })
    );
    for y in 0..10 {
        println!("");
        for x in 0..50 {
            if result_hash_set.contains(&Dot(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
    }
}
