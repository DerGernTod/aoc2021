use std::collections::{HashMap, HashSet};

pub fn part_1() {
    let highest_velocity = find_velocity((150, -86), (193, -136));
    let sum = sum_of_numbers(highest_velocity.1);
    print!("highest point reached at {} with velocity {:?} in {} steps", sum.0, highest_velocity, sum.1);
}

pub fn part_2() {
    let compatible_count = find_all_compatible_velocities((150, -86), (193, -136));
    print!("number of velocities: {}", compatible_count);
}

fn sum_of_numbers(num: i32) -> (i32, i32) {
    let cur = 0;
    let steps = 0;
    if num == 0 {
        return (cur, steps);
    }
    let (new_num, new_steps) = sum_of_numbers(num - 1);
    return (num + new_num, steps + new_steps + 1);
}

fn find_x_value_after_steps(mut velocity: i32, steps: i32) -> i32 {
    let mut cur_steps = 0;
    let mut target = 0;
    let acceleration = if velocity < 0 { 1 } else { - 1 };
    while velocity.abs() > 0 && cur_steps < steps {
        target += velocity;
        cur_steps += 1;
        velocity += acceleration;
    }
    target
}

fn find_for_x_axis(min: i32, max: i32, steps: i32) -> Option<i32> {
    let mut tried_velocity = max;
    while tried_velocity > 0 {
        let x = find_x_value_after_steps(tried_velocity, steps);
        if x <= max && x >= min  {
            return Some(tried_velocity);
        }
        tried_velocity -= 1;
    }
    None
}


fn find_velocity(target_area_start: (i32, i32), target_area_end: (i32, i32)) -> (i32, i32) {
    let (min_x, min_y) = target_area_start;
    let (max_x, max_y) = target_area_end;
    let mut best_y = max_y.abs() - 1;
    loop {
        let steps = best_y * 2 + 2;
        if let Some(x) = find_for_x_axis(min_x, max_x, steps) {
            return (x, best_y);
        }
        best_y = best_y - 1;
        if best_y < min_y {
            // TODO 
            println!("best y is not anymore in valid area!")
        }
    }
}

fn find_velocities_for_x_axis_with_steps(min: i32, max: i32, steps: i32) -> Vec<i32> {
    let mut tried_velocity = max;
    let mut result = vec![];
    while tried_velocity > 0 {
        let x = find_x_value_after_steps(tried_velocity, steps);
        if x <= max && x >= min {
            result.push(tried_velocity);
        }
        tried_velocity -= 1;
    }
    result
}

fn find_all_compatible_velocities((min_x, min_y): (i32, i32), (max_x, max_y): (i32, i32)) -> usize {
    let acceleration = -1;
    let mut y_velocities_per_steps = HashMap::new();
    
    for v_y_start in max_y..max_y.abs() {
        let mut cur_y = v_y_start;
        let mut cur_v_y = v_y_start;
        let mut num_steps = 1;
        while cur_y >= max_y {
            if cur_y <= min_y {
                println!("found valid y velocity {} after {} steps", v_y_start, num_steps);
                y_velocities_per_steps
                    .entry(num_steps)
                    .and_modify(|num: &mut Vec<i32>| num.push(v_y_start))
                    .or_insert(vec![v_y_start]);
            }
            cur_v_y += acceleration;
            cur_y += cur_v_y;
            num_steps += 1;
        }
    }
    let velocities = y_velocities_per_steps
        .into_iter()
        .fold(HashSet::new(),
            |set, (steps, velocities)| {
                velocities
                    .into_iter()
                    .fold(set, |set, v_y| 
                        find_velocities_for_x_axis_with_steps(min_x, max_x, steps)
                            .into_iter()
                            .fold(set, |mut set, v_x| {
                                set.insert((v_x, v_y));
                                set
                            }))
            }
    );
    println!("total y counts: {:?}", velocities);
    velocities.len()
    // let total_count = num_velocities_per_steps
    //     .into_iter()
    //     .fold(0, |total, (steps, count)| 
    //         total + find_count_for_x_axis_with_steps(min_x, max_x, steps) * count);

    // println!("total velocities: {}", total_count);
    // total_count
}

#[cfg(test)]
mod tests {
    use crate::day_17::*;
    #[test]
    fn test_find_steps_between() {

    }
    #[test]
    fn test_find_for_x_axis() {
        assert_eq!(find_for_x_axis(20, 30, 6), Some(6));
    }
    #[test]
    fn test_sum_of_numbers() {
        assert_eq!(sum_of_numbers(4), (10, 4));
    }
    #[test]
    fn part_1() {
        let highest_velocity = find_velocity((20, -5), (30, -10));
        assert_eq!(highest_velocity, (6, 9));
        assert_eq!(sum_of_numbers(highest_velocity.1).0, 45);
    }
    #[test]
    fn part_2() {
        assert_eq!(find_all_compatible_velocities((20, -5), (30, -10)), 112);
    }
}