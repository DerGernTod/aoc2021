use std::{collections::HashMap, fs, fmt};

use super::{cell::{Cell, CellType}, pod::{PodKind, Pod}, move_command::MoveCommand};

enum MoveResult {
    Blocked((u32, u32)),
    Free,
    Impossible
}

fn column_to_pod_kind(column: usize) -> PodKind {
    match column {
        3 => PodKind::Amber,
        5 => PodKind::Bronze,
        7 => PodKind::Copper,
        9 => PodKind::Desert,
        _ => panic!("Didn't expect this column index: {:?}!", column)
    }
}

fn char_to_pod_kind(pod_char: char) -> PodKind {
    match pod_char {
        'A' => PodKind::Amber,
        'B' => PodKind::Bronze,
        'C' => PodKind::Copper,
        'D' => PodKind::Desert,
        _ => panic!("Didn't expect this pod char: {:?}!", pod_char)
    }
}

fn map_char_to_cell(row: usize, column: usize, cur_char: char) -> ((u32, u32), Cell) {
    let location = (column as u32, row as u32);
    (location, match cur_char {
        '#' | ' ' => Cell::new(
            None, 
            CellType::Wall, 
            location),
        '.' if matches!(column, 3 | 5 | 7 | 9) => Cell::new(
            None,
            CellType::Entry,
            location),
        '.' => Cell::new(
            None,
            CellType::Hallway,
            location),
        x@'A'..='D' => Cell::new(
            Some(Pod::new(
                char_to_pod_kind(x),
                location
            )),
            CellType::Goal(column_to_pod_kind(column)),
            location),
        _ => panic!("Unexpected input: {:?}!", cur_char)
    })
}

pub struct Grid {
    pub cells: HashMap<(u32, u32), Cell>
}
impl Grid {
    pub fn new(path: &str) -> Grid {
        let read_in = fs::read_to_string(path).unwrap();
        let cells = read_in
            .split('\n')
            .into_iter()
            .enumerate()
            .flat_map(|(row, line)| line
                .chars()
                .into_iter()
                .enumerate()
                .map(move |(column, cur_char)| map_char_to_cell(row, column, cur_char))
            );
            
        Grid {
            cells: HashMap::from_iter(cells)
        }
    }

    fn try_move_pod(&mut self, from: &(u32, u32), to: &(u32, u32)) -> MoveResult {
        let [from_cell, to_cell] = self.cells.get_many_mut([from, to]).unwrap();
        let initiator_location = from_cell.location;
        if let Some(pod) = to_cell.occupant.as_ref() {
            let occupant_location = pod.location;
            if Some(initiator_location) == pod.is_pushed_by {
                MoveResult::Impossible
            } else {
                from_cell.occupant.as_mut().unwrap().is_pushed_by = Some(occupant_location);
                MoveResult::Blocked(occupant_location)
            }
        } else {
            let mut from_pod = from_cell.occupant.take();
            from_pod.as_mut().unwrap().location = *to;
            from_pod.as_mut().unwrap().is_pushed_by = None;
            to_cell.occupant = from_pod;
            println!("\n{:?}", &self);
            MoveResult::Free
        }
    }

    pub fn sort(&mut self) -> u32 {
        // get all pod locations
        let mut pods: Vec<&Pod> = self.cells
            .iter()
            .filter_map(|(_, cell)| cell.occupant.as_ref())
            .collect();
        // TODO calc permutations, D pods must always come first, but mix and match where necessary, set_y_goals
        pods.sort();
        let pod_locations: Vec<(u32, u32)> = pods.iter().map(|pod| pod.location).collect();

        // two possible start points: pod one to (9, 3) or pod two to (9, 3)
        let d_pod_two = self.cells.get(&pod_locations[1]).unwrap().occupant.as_ref().unwrap();
        let d_pod_two_goals = d_pod_two.calc_goals();

        let mut move_cmd = MoveCommand::new(d_pod_two.location, d_pod_two_goals);
        let mut steps = vec![];
        let mut possible_option = move_cmd.start_next_option();
        let mut cur_loc = d_pod_two.location;
        while let Some(_) = possible_option {
            
            while let Some(step) = move_cmd.step() {
                if let Some(pod) = self.cells.get(&step).unwrap().occupant.as_ref() {
                    // option invalid, pick next one
                    possible_option = move_cmd.start_next_option();
                    break;
                }
                println!("{:?} to {:?}", cur_loc, step);
                steps.push((cur_loc, step));
                cur_loc = step;
            }
            // option completed! move and pick next pod
        }
        // all options exhausted - pick next pod

        // all pods exhausted - check result - all in goal? calc cost, start with next pod as start

        12521
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![vec![String::from("#"); 13]; 5];
        for cell in self.cells.values() {
            let (col, row) = cell.location;
            lines[row as usize][col as usize] = cell.to_string();
        }
        
        let m = lines.into_iter().map(|line| line.join("")).collect::<Vec<String>>();
        write!(f, "{:}", m.join("\n"))
    }
}
impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}