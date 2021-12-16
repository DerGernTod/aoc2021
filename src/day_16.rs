use std::fs;

pub fn part_1() {
    let input = parse_to_bin("./input/day_16.txt");
    let mut cursor_pos = 0;
    let out = read_packet(&input, &mut cursor_pos, true);
    println!("{:?}", out);
    println!("{:?}", get_version_sum(out));
}

pub fn part_2() {
    let input = parse_to_bin("./input/day_16.txt");
    let mut cursor_pos = 0;
    let out = read_packet(&input, &mut cursor_pos, false);
    println!("{:?}", out);
}

fn get_version_sum(out: Vec<i64>) -> i64 {
    let mut next_is_version = false;
    let mut version_sum = 0;
    for result in out {
        if next_is_version {
            version_sum += result;
        }
        next_is_version = result == -1;
    }
    version_sum
}

fn parse_to_bin(file: &str) -> Vec<char> {
    let input_str = fs::read_to_string(file).unwrap();
    parse_from_string(&input_str)
}

fn parse_from_string(input_str: &str) -> Vec<char> {
    input_str
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|num| format!("{:04b}", num))
        .map(|formatted| formatted.chars().collect::<Vec<char>>())
        .flatten()
        .collect()
}

fn read_packet(input: &Vec<char>, cursor_pos: &mut usize, write_bounds: bool) -> Vec<i64>  {
    let (version, type_id) = read_headers(input, cursor_pos);
    let mut out = vec![];
    if write_bounds {
        out.push(-1);
        out.push(version.into());
    }
    let infos = match type_id {
        4 => read_literals(input, cursor_pos),
        x => read_operator(input, cursor_pos, x, write_bounds)
    };
    out.push(infos);
    if write_bounds {
        out.push(-2);
    }
    out
}

fn read_literals(input: &Vec<char>, cursor_pos: &mut usize) -> i64 {
    let mut out = vec![];
    loop {
        let signal_bit = input.get(*cursor_pos).unwrap();
        *cursor_pos += 1;
        let literal: String = input[*cursor_pos..*cursor_pos + 4].iter().collect();
        out.push(literal);
        *cursor_pos += 4;
        if *signal_bit == '0' {
            break;
        }
    }
    i64::from_str_radix(&out.join(""), 2).expect(&format!("Error parsing bool from string {:?}", &out.join("")))
}

fn read_operator(input: &Vec<char>, cursor_pos: &mut usize, operator_type: u8, write_bounds: bool) -> i64 {
    let mut out = vec![];
    let length_type_id = input.get(*cursor_pos).unwrap();
    *cursor_pos += 1;
    match length_type_id {
        '0' => {
            let sub_packet_length: String = input[*cursor_pos..*cursor_pos + 15].iter().collect();
            let sub_packet_length = usize::from_str_radix(&sub_packet_length, 2).unwrap();
            *cursor_pos += 15;
            let start_cursor = *cursor_pos;
            while *cursor_pos < start_cursor + sub_packet_length {
                let mut infos = read_packet(input, cursor_pos, write_bounds);
                out.append(&mut infos);
            }
        },
        '1' => {
            let num_sub_packets: String = input[*cursor_pos..*cursor_pos + 11].iter().collect();
            let num_sub_packets = i32::from_str_radix(&num_sub_packets, 2).unwrap();
            *cursor_pos += 11;
            for _ in 0..num_sub_packets {
                let mut infos = read_packet(input, cursor_pos, write_bounds);
                out.append(&mut infos);
            }
        },
        _ => panic!("There was a non-binary character in this operator!")
    };
    let out_iter = out.iter();
    match operator_type {
        0 => out_iter.sum(),
        1 => out_iter.product(),
        2 => *out_iter.min().unwrap(),
        3 => *out_iter.max().unwrap(),
        5 => if out[0] > out[1] { 1 } else { 0 },
        6 => if out[0] < out[1] { 1 } else { 0 },
        7 => if out[0] == out[1] { 1 } else { 0 },
        _ => panic!("Found an invalid operator type!")
    }
}

fn read_headers(input: &Vec<char>, cursor_pos: &mut usize) -> (u8, u8) {
    let version: String = input[*cursor_pos..*cursor_pos + 3].iter().collect();
    let version = u8::from_str_radix(&version, 2).unwrap();
    *cursor_pos += 3;
    let type_id: String = input[*cursor_pos..*cursor_pos + 3].iter().collect();
    let type_id = u8::from_str_radix(&type_id, 2).unwrap();
    *cursor_pos += 3;
    (version, type_id)
}


#[cfg(test)]
mod tests {
    use crate::day_16::*;
    #[test]
    fn part_1() {
        let input = parse_from_string("A0016C880162017C3686B18A3D4780");
        let mut cursor_pos = 0;
        let out = read_packet(&input, &mut cursor_pos, true);
        
        assert_eq!(get_version_sum(out), 31);
    }
    #[test]
    fn part_2() {
        let input = parse_from_string("9C0141080250320F1802104A08");
        let mut cursor_pos = 0;
        let out = read_packet(&input, &mut cursor_pos, false);
        assert_eq!(out[0], 1);
    }
}