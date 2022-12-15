use std::cmp::Ordering;
use std::collections::{BTreeMap};
use std::fmt::Display;
use std::rc::Rc;

use super::coords::Coords;
use super::pod::Pod;

#[derive(Debug, Clone)]
pub struct State {
    pub pods: BTreeMap<Coords, Pod>,
    pub cheapest_path: usize,
    pub visited: bool
}

impl State {
    pub fn new(pods: BTreeMap<Coords, Pod>, cheapest_path: usize) -> State {
        State {
            pods,
            cheapest_path,
            visited: false
        }
    }

    pub fn calc_possible_neighbors(&self, use_size_four: bool) -> Vec<Rc<State>> {
        self.pods
            .iter()
            .flat_map(|(coords, pod)| pod.cur_goals
                .iter()
                .filter_map(|goal| self.can_reach_goal(coords, goal).map(|steps| (goal, steps)))
                .map(|(goal, steps)| {
                    // create new state here
                    let mut pods_clone = self.pods.clone();
                    let mut pod = pods_clone.remove(coords).unwrap();
                    let pod_kind = pod.kind as usize;
                    pod.location = *goal;
                    pod.num_turns += 1;
                    pod.recalculate_goals(use_size_four);
                    pods_clone.insert(*goal, pod);
                    Rc::new(State::new(pods_clone, self.cheapest_path + steps * pod_kind))
                })
            )
            .collect()
    }

    fn can_reach_goal(&self, start: &Coords, goal: &Coords) -> Option<usize> {
        let mut cur_pos = *start;
        let mut steps = 0;
        while let Some(next_pos) = calc_next_step(&cur_pos, goal) {
            if self.pods.get(&next_pos).is_some() {
                return None;
            }
            cur_pos = next_pos;
            steps += 1
        }
        Some(steps)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec![];
        for y in 1..4 {
            for x in 1..12 {
                if let Some(pod) = self.pods.get(&(x, y)) {
                    chars.push(pod.kind.to_char());
                } else {
                    match (x, y) {
                        (x, 1) if x > 0 && x < 12 => chars.push('.'),
                        (x, 2 | 3) if x == 3 || x == 5 || x == 7 || x == 9 => chars.push('.'),
                        _ => chars.push('#')
                    }
                }
            }
            chars.push('\n');
        }
        
        write!(f, "{:}", chars.into_iter().collect::<String>())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pods == other.pods
    }
}

fn calc_next_step(start: &Coords, goal: &Coords) -> Option<Coords> {
    let (x, y) = *start;
    let (goal_x, goal_y) = *goal;
    if x == goal_x {
        match y.cmp(&goal_y) {
            Ordering::Less => Some((x, y + 1)),
            Ordering::Equal => None,
            Ordering::Greater => Some((x, y - 1)),
        }
    } else if y > 1 {
        Some((x, y - 1))
    } else if x < goal_x {
        Some((x + 1, y))
    } else if x > goal_x {
        Some((x - 1, y))
    } else {
        None
    }
}