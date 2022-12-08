use std::{collections::HashMap, fs, fmt, vec, cmp::Ordering};

use super::{
    cell::{Cell, CellType},
    pod::{PodKind, Pod, PodFactory, calc_next_step, self},
    transformation::Transform
};

pub enum PodMoveResultDec{
    Hit(usize),
    Pass(u32, Vec<Transform>),
    ReachedEnd(u32, Vec<Transform>)
}

enum PodMoveResult {
    Obstructed((u32, Vec<Transform>)),
    Solo((u32, Vec<Transform>)),
    None
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

fn build_cell_and_pod(row: u32, column: u32, cur_char: char, pod_factory: &mut PodFactory) -> (Cell, Option<Pod>) {
    let location = (column, row);
    let cell_type = match cur_char {
        '#' | ' ' => CellType::Wall,
        '.' if matches!(column, 3 | 5 | 7 | 9) => CellType::Entry,
        '.' => CellType::Hallway,
        x@'A'..='D' => CellType::Goal(char_to_pod_kind(x)),
        _ => panic!("Unexpected input: {:?}!", cur_char)
    };
    let (cell, pod) = if let CellType::Goal(pod_kind) = cell_type {
        let cur_pod = pod_factory.new_pod(pod_kind, location);

        (Cell::new(Some(cur_pod.id), cell_type, location), Some(cur_pod))
    } else {
        (Cell::new(None, cell_type, location), None)
    };
    (cell, pod)
}

pub struct Grid {
    cells: HashMap<(u32, u32), Cell>,
    pub pods: HashMap<usize, Pod>,
    hallway_pods: Vec<usize>,
    untouched_pods: Vec<usize>,
    iterations: Vec<(usize, Vec<Transform>)>
}
impl Grid {
    pub fn new(path: &str) -> Grid {
        let mut pod_factory = PodFactory::new();
        let read_in = fs::read_to_string(path).unwrap();

        // create cells and pods from input
        let (entries, pods) = read_in
            .split('\n')
            .into_iter()
            .enumerate()
            .flat_map(move |(row, line)| line
                .chars()
                .into_iter()
                .enumerate()
                .map(move |(column, cur_char)| (row as u32, column as u32, cur_char))
            )
            .fold((vec![], vec![]), |(mut entries, mut pods), (row, column, cur_char)| {
                let (cell, pod) = build_cell_and_pod(row, column, cur_char, &mut pod_factory);
                entries.push(((column, row), cell));
                if let Some(pod) = pod {
                    pods.push((pod.id, pod));
                }
                (entries, pods)
            });

        // store untouched pod ids sorted
        let pods = HashMap::from_iter(pods);   
        let mut pod_kind_tuples: Vec<(PodKind, usize)> = pods
            .values()
            .map(|pod| (pod.kind, pod.id))
            .collect();
        pod_kind_tuples.sort_by_key(|(kind, _)| *kind);
        let untouched_pods: Vec<usize> = pod_kind_tuples.into_iter().map(|(_, id)| id).collect();

        Grid {
            cells: HashMap::from_iter(entries),
            pods,
            hallway_pods: vec![],
            untouched_pods,
            iterations: vec![]
        }
    }

    pub fn get_iteration_len(&self) -> usize {
        self.iterations.len()
    }

    pub fn calc_goals_for_pod(&self, pod_id: usize) -> Vec<(u32, (u32, u32))> {
        let pod = self.pods.get(&pod_id).unwrap();
        let calced_goals = pod.calc_goals();
        calced_goals.into_iter().filter(|(_, (x, y))| {
            if x != &pod.kind.goal_col() {
                return true;
            } else if *y == 2 {
                if let Some(occupant) = self.cells.get(&(*x, 3)).unwrap().occupant {
                    return self.pods.get(&occupant).unwrap().kind == pod.kind
                } else {
                    return false;
                }
            }
            false
        }).collect()
    }

    pub fn get_pod_kinds_sorted(&self) -> Vec<(PodKind, usize)> {
        let mut pod_kind_tuples: Vec<(PodKind, usize)> = self.pods
            .values()
            .map(|pod| (pod.kind, pod.id))
            .collect();
        pod_kind_tuples.sort_by_key(|(kind, _)| *kind);
        pod_kind_tuples
    }

    pub fn follow_to_goal_dec(&self, pod_id: usize, goal_loc: (u32, u32)) -> PodMoveResultDec {
        let pod = self.pods.get(&pod_id).unwrap();
        let mut cur_pod_loc = pod.location;
        let mut transforms = vec![];
        let move_cost = pod.kind.val();
        while let Some(next_step) = calc_next_step(cur_pod_loc, goal_loc) {
            if self.is_pod_in_goal(pod_id) {
                return PodMoveResultDec::ReachedEnd(move_cost * transforms.len() as u32, transforms);
            }
            let step_cell = self.cells.get(&next_step).unwrap();
            if let Some(occupant_id) = step_cell.occupant {
                return PodMoveResultDec::Hit(occupant_id);
            } else {
                transforms.push(Transform(move_cost, cur_pod_loc, next_step));
                cur_pod_loc = next_step;
            }
        }

        PodMoveResultDec::Pass(move_cost * transforms.len() as u32, transforms)
    }

    pub fn is_pod_in_goal(&self, pod_id: usize) -> bool {
        let pod = self.pods.get(&pod_id).unwrap();
        let (pod_x, pod_y) = pod.location;
        if pod.is_in_goal_area() {
            if pod_y == 2 {
                let lower_cell = self.cells.get(&(pod_x, pod_y + 1)).unwrap();
                if let Some(occupant_id) = lower_cell.occupant {
                    self.pods.get(&occupant_id).unwrap().kind == pod.kind
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
        
    }

    fn reset_blocked_pod_goals(&mut self) {
        for pod in self.pods.values_mut() {
            pod.blocked_goals.clear();
        }
    }

    pub fn find_incomplete_pods(&self) -> Vec<usize> {
        let mut incomplete_pods: Vec<&Pod> = self.pods
            .iter()
            .filter(|&(id, pod)| !self.is_pod_in_goal(*id) && pod.walked_count < 2)
            .map(|(_, pod)| pod)
            .collect();
        incomplete_pods.sort_by(|pod_a, pod_b| {
            if pod_a.location.1 == pod_b.location.1 {
                pod_b.kind.cmp(&pod_a.kind)
            } else {
                pod_a.location.1.cmp(&pod_b.location.1)
            }
        });
        incomplete_pods.iter().map(|pod| pod.id).collect()
    }

    fn move_pod(&mut self, from: &(u32, u32), to: &(u32, u32)) {
        let from_cell = self.cells.get_mut(from).unwrap();
        let pod_id = from_cell.occupant.take().unwrap_or_else(|| panic!("Origin cell {:?} must be occupied! Tried to move to {:?}.", from, to));
        let to_cell = self.cells.get_mut(to).unwrap();
        if to_cell.occupant.is_some() {
            panic!("Target cell {:?} must not be occupied! Is occupied by {:?}", to, to_cell.occupant);
        }
        to_cell.occupant = Some(pod_id);
        let pod = self.pods.get_mut(&pod_id).unwrap();
        pod.location = *to;
        if to.1 != 1 {
            if let Some(pod_index) = self.hallway_pods.iter().position(|id| *id == pod_id) {
                self.hallway_pods.remove(pod_index);
            }
        } else if !self.hallway_pods.contains(&pod_id) {
            self.hallway_pods.push(pod_id);
        }
    }

    pub fn reverse_iterations(&mut self, count: usize) -> Vec<(usize, Vec<Transform>)> {
        let mut reversed_iterations = vec![];
        for _ in 0..count {
            let (pod_id, transforms) = self.iterations.pop().unwrap();
            self.pods.get_mut(&pod_id).unwrap().walked_count -= 1;
            for Transform(_, from, to) in transforms.iter().rev() {
                self.move_pod(to, from);
            }
            if !transforms.is_empty() {
                println!("{self}\n");
            }
            reversed_iterations.push((pod_id, transforms));
        }
        reversed_iterations
    }

    pub fn apply_iterations(&mut self, iterations: Vec<(usize, Vec<Transform>)>) {
        for iteration in iterations.into_iter().rev() {
            let (pod_id, transforms) = &iteration;
            self.pods.get_mut(pod_id).unwrap().walked_count += 1;
            for Transform(_, from, to) in transforms {
                self.move_pod(from, to);
            }
            self.iterations.push(iteration);
        }
    }

    pub fn exec_transformations(&mut self, transformations: &[Transform], pod_id: usize) {
        self.iterations.push((pod_id, transformations.to_owned()));
        self.pods.get_mut(&pod_id).unwrap().walked_count += 1;
        for Transform(_, from, to) in transformations.iter() {
            self.move_pod(from, to);
        }
    }
    fn calc_remaining_pods(&self) -> Vec<usize> {
        self.untouched_pods
            .clone()
            .into_iter()
            .filter(|pod_id| !self.is_pod_in_goal(*pod_id))
            .collect()
    }

}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![vec![String::from("#"); 13]; 5];
        for cell in self.cells.values() {
            let (col, row) = cell.location;
            let representation = cell.occupant
                .map(|id| self.pods.get(&id).unwrap().to_string())
                .unwrap_or(cell.to_string());
            lines[row as usize][col as usize] = representation;
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