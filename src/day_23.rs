use crate::day_23::Pod::*;
pub fn part_1() {
    let input = [(Bronze, Copper), (Bronze, Amber), (Desert, Desert), (Amber, Copper)];

    println!("{} energy required for sorting!", sort_pods(input));
}

pub fn part_2() {

}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pod {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000
}

fn sort_pods(target: [(Pod, Pod); 4]) -> u64 {
    // start with origin and calc destinations (= input)
    let rooms = [
        (Some(Amber), Some(Amber)),
        (Some(Bronze), Some(Bronze)),
        (Some(Copper), Some(Copper)),
        (Some(Desert), Some(Desert))];
    let hallway = [None as Option<Pod>; 10];
    while let Some(cur_target) = calc_cur_target(target, rooms) {
        // continue sorting
    }
    12521
}

fn calc_cur_target(target: [(Pod, Pod); 4], rooms: [(Option<Pod>, Option<Pod>); 4]) -> Option<(usize, u8)> {
    rooms.into_iter().enumerate().find_map(|(index, room)| 
        match room {
            (Some(top), Some(bottom)) => if target[index] == (top, bottom) { None } else { None },
            _ => None
        }
    )
}

#[cfg(test)]
mod tests {
    use crate::day_23::*;
    use crate::day_23::Pod::*;

    #[test]
    fn test_part_1() {
        let test_input = [(Bronze, Amber), (Copper, Desert), (Bronze, Copper), (Desert, Amber)];
        assert_eq!(sort_pods(test_input), 12521);

    }
}