use std::collections::HashMap;

pub fn part_1() {
    let highest_velocity = find_velocity((150, -86), (193, -136));
    let sum = sum_of_numbers(highest_velocity.1);
    print!("highest point reached at {} with velocity {:?} in {} steps", sum.0, highest_velocity, sum.1);
}

pub fn part_2() {

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


fn find_x_value_after_steps(mut velocity: i32, steps: i32) -> Option<i32> {
    let mut cur_steps = 0;
    let mut target = 0;
    let acceleration = if velocity < 0 { 1 } else { - 1 };
    while velocity.abs() > 0 && cur_steps < steps {
        target += velocity;
        cur_steps += 1;
        velocity += acceleration;
    }
    if cur_steps == steps || velocity.abs() == 0 { Some(target) } else { None }
}

fn find_for_x_axis(min: i32, max: i32, steps: i32) -> Option<i32> {
    let mut tried_velocity = 1;
    for _ in 0..50 {
        match find_x_value_after_steps(tried_velocity, steps) {
            Some(x) if x <= max && x >= min => return Some(tried_velocity),
            _ => ()
        }
        tried_velocity += 1;
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

fn find_all_compatible_velocities((min_x, min_y): (i32, i32), (max_x, max_y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut result = vec![];
    let mut iteration = 0;
    let mut best_y = max_y.abs() - 1;
    let acceleration = -1;
    let mut steps_per_velocity = HashMap::new();
    for v_y_start in max_y..max_y.abs() {
        let mut cur_y = v_y_start;
        let mut cur_v_y = v_y_start;
        let mut valid_steps = vec![];
        let mut num_steps = 1;
        while cur_y >= max_y {
            if cur_y <= min_y {
                valid_steps.push(num_steps);
            }
            cur_v_y += acceleration;
            cur_y += cur_v_y;
            num_steps += 1;
        }
        steps_per_velocity.insert(v_y_start, valid_steps);
    }
    // found all y positions incl. steps that are possible
    // todo: find all matching x positions with these amounts of steps

    // loop {
    //     let steps = best_y * 2 + 2 + iteration;
    //     if let Some(x) = find_for_x_axis(min_x, max_x, steps) {
    //         result.push((x, best_y));
    //     }
    //     best_y = best_y - 1;
    //     if best_y < min_y {
    //         iteration += 1;
    //         best_y = max_y / (iteration + 1);
    //         // TODO 
    //         println!("best y is not anymore in valid area!")
    //     }
    //     if 
    // }
    println!("{:?}", steps_per_velocity);
    result
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
        find_all_compatible_velocities((20, -5), (30, -10));
    }
}