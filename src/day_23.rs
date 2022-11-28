mod pod;
mod cell;
mod command;
mod grid;
mod move_command;
use self::grid::Grid;

/**
 * create stack of movements. determine where highest cost pods want to go. occupied?
 * if yes, that's the next priority. put movement of highest cost pods on stack,
 * determine where next priority has to go. move step after step. occupied?
 * ...
 * calculate cost of path based on cost of paths of pods required to move
 * movement: how to determine where to go? grid with cells.
 *  cell state: 
 *      occupant: Option<Occupant>, Occupant = Wall,Pod
 *      cell_type: CellTypes = Hallway, Goal<PodKind>, Entry
 *  pod:
 *      allowed_targets: Hallway, Goal X
 *      location: (x, y)
 *      pod_kind: PodKind = Amber, Bronze, Copper, Desert
 *      commands: Command[] = Up, Down, Left, Right, Wait      
 * 
 *      
 */

pub fn part_1() {

    println!("{} energy required for sorting!", sort_pods(Grid::new("../input/day_23.txt")));
}

pub fn part_2() {

}


fn sort_pods(mut grid: Grid) -> u32 {
    // build map
    grid.sort()
}


#[cfg(test)]
mod tests {
    use crate::day_23::*;

    #[test]
    fn test_part_1() {
        assert_eq!(sort_pods(Grid::new("./input/day_23.test.txt")), 12521);
    }
}