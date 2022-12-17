pub fn day_24() {
    println!("Manually reverse engineer the function and calculate by hand ;)");
    println!("For the sake of showing that it does something, here's the result for the first round: {:?}",
        reverse_engineered(1, 0, 0));
}
/*
  calculation results per digit for z (same for min and max):
    z1:     w1 + 6
    z2:     26w1 + w2 + 170
    z3:     26*26w1 + 26w2 + w3 + 4433
    z4:     26w1 + w2 + 170             => resolves w3 and w4
    z5:     26*26w1 + 26*w2 + w5 + 4426
    z6:     26w1 + w2 + 170             => resolves w5 and w6
    z7:     w1 + 6                      => resolves w2 and w7
    z8:     26*w1 + w8 + 159
    z9:     w1 + 6                      => resolves w8 and w9
    z10:    26w1 + w10 + 170
    z11:    26*26w1 + 26*w10 + w11 + 4424
    z12:    26w1 + w10 + 170            => resolves w11 and w12
    z13:    w1 + 6                      => resolves w10 and w13
    z14:    0                           => resolves w1 and w14
 */

fn reverse_engineered(w: i64, mut z: i64, exec_count: usize) -> Option<i64> {
    let mut x = z % 26;
    if matches!(exec_count, 4 | 6 | 7 | 9 | 12 | 13 | 14) {
        z /= 26;
    }
    x += match exec_count {
        1 | 2 => 11,
        3 | 11 => 15,
        4 => -14,
        5 => 10,
        6 => 0,
        7 => -6,
        8 | 10 => 13,
        9 => -3,
        12 | 14 => -2,
        13 => -9,
        _ => panic!("Invalid exec count: {exec_count}")
    };
    if x == w {
        return Some(z);
    }
    x = (x != w) as i64; 
    x = (x != 0) as i64;
    z = z.checked_mul(25 * x + 1)?;
    let mut y = w; 
    y += match exec_count {
        1 | 5 | 7 => 6,
        2 | 10 => 14,
        3 | 6 => 13,
        4 | 14 => 1,
        8 => 3,
        9 => 8,
        11 => 4,
        12 => 7,
        13 => 15,
        _ => panic!("Invalid exec count: {exec_count}")
    };
    Some(x * y + z)
}

#[cfg(test)]
mod tests {
    use super::reverse_engineered;

    #[test]
    fn test_part_1() {
        let input_num = 51983999947999;
        let inputs = number_to_vec_rev(input_num).unwrap();
        assert_eq!(Some(0), execute_for_inputs(inputs));
    }
    #[test]
    fn test_part_2() {
        let input_num = 11211791111365;
        let inputs = number_to_vec_rev(input_num).unwrap();
        assert_eq!(Some(0), execute_for_inputs(inputs));
    }
    #[test]
    fn test_brute_force() {
        assert_eq!(find_largest_monad_valid(), 51983999947999);
    }

        
    fn execute_for_inputs(mut values: Vec<i32>) -> Option<i64> {
        let mut z = 0;
        let mut round = 0;
        while let Some(val) = values.pop() {
            round += 1;
            if let Some(cur_z) = reverse_engineered(val as i64, z, round) {
                z = cur_z;
            } else {
                return None;
            }
        }
        Some(z)
    }

    fn number_to_vec_rev(n: usize) -> Option<Vec<i32>> {
        n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .map(|c| if c == 0 { None } else { Some(c) })
            .rev()
            .collect()
    }

    // probably computers in 30 years can brute force this in a reasonable time :D
    fn find_largest_monad_valid() -> usize {
        let mut lowest = i64::MAX;
        for val in (11111111111111..99999999999999).rev() {
            if let Some(vec) = number_to_vec_rev(val) {
                if let Some(result) = execute_for_inputs(vec) {
                    if result < lowest {
                        println!("Found new lowest: {result} with number {val}");
                        lowest = result;
                    }
                    if result == 0 {
                        return val;
                    }
                }
            }
        }
        0
    }
}