mod pod;
mod cell;
use std::collections::HashMap;

use self::cell::Cell;
use self::pod::{PodKind::{self, Amber, Bronze, Copper, Desert}, Pod, RoomType};
pub fn part_1() {
    let input = [(Bronze, Copper), (Bronze, Amber), (Desert, Desert), (Amber, Copper)];

    println!("{} energy required for sorting!", sort_pods(input));
}

pub fn part_2() {

}

fn build_map() -> HashMap<(u32, u32), Cell> {
    HashMap::from([
        // hallway
        ((0, 0), Cell::new(None, false, false, false, true)),
        ((1, 0), Cell::new(None, false, false, true, true)),
        ((2, 0), Cell::new(None, false, true, true, true)),
        ((3, 0), Cell::new(None, false, false, true, true)),
        ((4, 0), Cell::new(None, false, true, true, true)),
        ((5, 0), Cell::new(None, false, false, true, true)),
        ((6, 0), Cell::new(None, false, true, true, true)),
        ((7, 0), Cell::new(None, false, false, true, true)),
        ((8, 0), Cell::new(None, false, true, true, true)),
        ((9, 0), Cell::new(None, false, false, true, true)),
        ((10, 0), Cell::new(None, false, false, true, false)),
        // amber
        ((2, 1), Cell::new(None, true, true, false, false)),
        ((2, 2), Cell::new(None, true, false, false, false)),
        // bronze
        ((4, 1), Cell::new(None, true, true, false, false)),
        ((4, 2), Cell::new(None, true, false, false, false)),
        // copper
        ((6, 1), Cell::new(None, true, true, false, false)),
        ((6, 2), Cell::new(None, true, false, false, false)),
        // desert
        ((8, 1), Cell::new(None, true, true, false, false)),
        ((8, 2), Cell::new(None, true, false, false, false)),
    ])
}

fn sort_pods(target: [(PodKind, PodKind); 4]) -> u64 {
    // build map
    let mut map = build_map();
    let mut pods = [
        Pod::new(Amber, (2, 1)),
        Pod::new(Amber, (2, 2)),
        Pod::new(Bronze, (4, 1)),
        Pod::new(Bronze, (4, 2)),
        Pod::new(Copper, (6, 1)),
        Pod::new(Copper, (6, 2)),
        Pod::new(Desert, (8, 1)),
        Pod::new(Desert, (8, 2))
    ];
    for pod in pods {
        map.get_mut(&pod.coords).unwrap().occupant = Some(pod);
    }
    // start with origin and calc destinations (= input)
    
    while let Some((cur_state, hallway)) = calc_cur_state(target, rooms, hallway) {
        // continue sorting
    }
    12521
}

fn calc_cur_state(target: [(PodKind, PodKind); 4], rooms: [(Option<Pod>, Option<Pod>); 4], hallway: [Option<Pod>; 7]) -> Option<((usize, u8), [Option<Pod>; 7])> {
    // check if first room is complete
    if rooms[0] != (Some(target[0].0), Some(target[1].0)) {
        // if not, try to move the outer one... determine its target room

    }
    None
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