mod pod;
mod cell;
mod grid;
mod transformation;
mod decision_tree;
use self::{grid::Grid, decision_tree::DecisionTree};

pub fn part_1() {

    println!("{} energy required for sorting!", sort_pods(Grid::new("../input/day_23.txt")));
}

pub fn part_2() {

}


fn sort_pods(mut grid: Grid) -> u32 {
    let pods = grid.get_pod_kinds_sorted();
    let mut trees = DecisionTree::from_pods(pods);
    // build map
    //grid.sort()
    let result_paths: Vec<u32> = trees
        .iter_mut()
        .filter_map(|tree| tree.evaluate(&mut grid))
        .collect();
    result_paths.iter().sum()    
}


#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::day_23::*;

    #[test]
    fn test_part_1() {
        thread::sleep(Duration::from_secs(1));
        assert_eq!(sort_pods(Grid::new("./input/day_23.test.txt")), 12521);
    }
}
/*
#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########

*/