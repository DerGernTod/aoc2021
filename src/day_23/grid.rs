use std::{collections::HashMap, fs, fmt, vec};

use super::{
    cell::{Cell, CellType},
    pod::{PodKind, Pod, PodFactory, calc_next_step},
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
    iteration_id: u32
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
            iteration_id: 0
        }
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
        incomplete_pods.sort_by_key(|pod| pod.kind);
        incomplete_pods.iter().rev().map(|pod| pod.id).collect()
    }

    fn move_pod(&mut self, from: &(u32, u32), to: &(u32, u32), is_reverse: bool) {
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

    fn reverse_transformations(&mut self, transformations: &[Transform]) {
        for Transform(_, to, from) in transformations.iter().rev() {
            self.move_pod(from, to, true);
        }
    }

    pub fn exec_transformations(&mut self, transformations: &[Transform], reverse: bool) {
        if reverse {
            for Transform(_, from, to) in transformations.iter().rev() {
                self.move_pod(to, from, reverse);
            }
        } else {
            for Transform(_, from, to) in transformations.iter() {
                self.move_pod(from, to, reverse);
            }
        }
    }

    fn follow_to_goal(&mut self, pod_id: usize, goal_loc: (u32, u32), child_skip_count: u32) -> PodMoveResult {
        let mut transformations: Vec<Transform> = vec![];

        let pod = self.pods.get(&pod_id).unwrap();
        let mut cur_pod_loc = pod.location;
        let move_price = pod.kind.val();

        while let Some(next_step) = calc_next_step(cur_pod_loc, goal_loc) {
            if self.is_pod_in_goal(pod_id) {
                break;
            }
            let step_cell = self.cells.get_mut(&next_step).unwrap();
            // 3.2.1 hit something? pick that pod and go to 2.
            if let Some(occupant_id) = step_cell.occupant {
                self.reverse_transformations(&transformations);
                return self.lead_pod_to_goal(occupant_id, child_skip_count)
                    .map(PodMoveResult::Obstructed)
                    .unwrap_or(PodMoveResult::None);
            } else {
                transformations.push(Transform(move_price, cur_pod_loc, next_step));
                self.move_pod(&cur_pod_loc, &next_step, false);
                cur_pod_loc = next_step;
            }
        }
        let (cur_x, cur_y) = cur_pod_loc;
        if cur_y == 2 {
            // if we're in goal area, check lower goal
            let lower_loc = (cur_x, cur_y + 1);
            // if it's of a different kind return None
            if let Some(occupant) = self.cells.get(&lower_loc).unwrap().occupant {
                let occupant_pod = self.pods.get(&occupant).unwrap();
                if self.pods.get(&pod_id).unwrap().kind != occupant_pod.kind {
                    self.reverse_transformations(&transformations);
                    return PodMoveResult::None;
                }
            // if it's empty, move there
            } else {
                transformations.push(Transform(move_price, cur_pod_loc, lower_loc));
                self.move_pod(&cur_pod_loc, &lower_loc, false);
            }
        }
        PodMoveResult::Solo((child_skip_count, transformations))
    }

    fn lead_pod_to_goal(&mut self, pod_id: usize, skip_count: u32) -> Option<(u32, Vec<Transform>)> {
        if self.is_pod_in_goal(pod_id) {
            return Some((skip_count, vec![]));
        }
        let pod = self.pods.get(&pod_id).unwrap();
        if pod.is_in_goal_area() && pod.walked_count > 0 {
            return None;
        }
        let mut skip_count = skip_count;
        while !self.is_pod_in_goal(pod_id) && skip_count < 10 {
            if let Some((_, mut transforms_first)) = self.handle_pod(pod_id, skip_count) {
                if let Some((_, mut transforms_second)) = self.handle_pod(pod_id, 0) {
                    transforms_first.append(&mut transforms_second);
                    return Some((skip_count, transforms_first));
                } else {
                    self.reverse_transformations(&transforms_first);
                }
            }
            skip_count += 1;
            println!("increasing skip count for pod {} to {}", pod_id, skip_count);
        }
        None
    }

    fn handle_pod(&mut self, pod_id: usize, skip_count: u32) -> Option<(u32, Vec<Transform>)> {
        // 2. calc all ways to goal including hallway checkpoints and order by cost.
        if self.is_pod_in_goal(pod_id) {
            return Some((skip_count, vec![]));
        }
        let pod = self.pods.get(&pod_id).unwrap();
        let mut goals = pod.calc_goals();
        for _ in 0..skip_count {
            goals.pop();
        }
        let mut cur_transforms: Vec<Transform> = vec![];
        let mut child_skip_counts: Vec<u32> = vec![];
        let mut cur_child_skip = 0;
        // 3. pick next way
        while let Some(goal) = goals.pop() {
            if self.is_pod_in_goal(pod_id) {
                return Some((skip_count, cur_transforms));
            }
            // make sure to not use the same goal that was used in this iteration again
            if self.pods.get(&pod_id).unwrap().blocked_goals.contains(&goal) {
                continue;
            }
            self.pods.get_mut(&pod_id).unwrap().blocked_goals.insert(goal);
            let (_, goal_loc) = goal;

            // 3.2 way? follow
            match self.follow_to_goal(pod_id, goal_loc, cur_child_skip) {
                PodMoveResult::Obstructed((child_skip_count, mut transforms)) => {
                    cur_transforms.append(&mut transforms);
                    child_skip_counts.push(child_skip_count);
                    goals = self.pods.get(&pod_id).unwrap().calc_goals();
                },
                PodMoveResult::Solo((child_skip_count, mut transforms)) => {
                    cur_transforms.append(&mut transforms);
                    println!("iteration id: {}\n{:?}\n", self.iteration_id, &self); 
                    self.iteration_id += 1;
                    self.pods.get_mut(&pod_id).unwrap().blocked_goals.remove(&goal);
                    return Some((child_skip_count, cur_transforms));
                },
                PodMoveResult::None => {
                },
            }
            self.pods.get_mut(&pod_id).unwrap().blocked_goals.remove(&goal);
            
            if cur_child_skip < 9 && goals.is_empty() {
                if let Some(child_skip) = child_skip_counts.pop() {
                    cur_child_skip = child_skip + 1;
                    if cur_child_skip < 9 {
                        goals = self.pods.get(&pod_id).unwrap().calc_goals();
                    }
                }
            }
        }
        // 3.1 no way? return err, cycle pod list, go to 1.
        None
    }
    
    fn calc_remaining_pods(&self) -> Vec<usize> {
        self.untouched_pods
            .clone()
            .into_iter()
            .filter(|pod_id| !self.is_pod_in_goal(*pod_id))
            .collect()
    }

    pub fn sort(&mut self) -> u32 {
        println!("{:?}\n", &self);
        let mut transformations: Vec<Transform> = vec![];
        let mut transformations_stack: Vec<Vec<Transform>> = vec![];
        // 1. pick next pod in hallway or remaining
        let mut remaining_pods: Vec<usize> = self.untouched_pods.clone();
        let mut skip_counts: HashMap<usize, u32> = HashMap::new();
        let mut pod_id_queue = vec![];
        while let Some(pod_id) = remaining_pods.pop() {
            if self.is_pod_in_goal(pod_id) {
                continue;
            }
            let cur_skip_count = *skip_counts.entry(pod_id).or_insert(0);
            println!("try handling pod {} with skip count {}", pod_id, cur_skip_count);
            if let Some((skip_count, path)) = self.lead_pod_to_goal(pod_id, cur_skip_count) {
                transformations_stack.push(path);
                skip_counts.insert(pod_id, skip_count);
                pod_id_queue.push(pod_id);
            } else {
                if let Some(transform) = transformations_stack.pop() {
                    self.reverse_transformations(&transform);
                }
                if cur_skip_count == 0 {
                    // no path at all! roll back previous too
                    if let Some(popped_pod_id) = pod_id_queue.pop() {
                        let popped_skip_count = skip_counts.get(&popped_pod_id).unwrap();
                        skip_counts.insert(popped_pod_id, popped_skip_count + 1);
                    }
                    if let Some(transform) = transformations_stack.pop() {
                        self.reverse_transformations(&transform);
                    }
                }
                remaining_pods = self.calc_remaining_pods();
            }
            self.reset_blocked_pod_goals();
        }
        
        // 3.2.1 hit something? pick that pod and go to 2.
        // 3.2.1.1 err? go to 3
        // 3.2.1.2 ok? follow and go to 3.2.1
        // 3.2.2 hit checkpoint? add to hallway list, then go to 1
        // 3.2.3 reached goal area? check what's in next goal area
        // 3.2.3.1 occupied by none-sibling? return err
        // 3.2.3.2 return ok, remove pod from remaining pod list, go to 1
        println!("Transformation complete in {:?} steps", transformations.len());
        let mut cost = 0;
        while let Some(transform) = transformations.pop() {
            self.move_pod(&transform.2, &transform.1, true);
            cost += transform.0;
            //println!("{:?}\n", &self);
        }
        cost
        //transformations.into_iter().map(|transform| transform.0).sum()
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