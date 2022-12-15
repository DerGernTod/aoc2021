use super::coords::Coords;
use std::{hash::Hash, cmp::Ordering};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum PodKind {
    Amber = 1,
    Bronze = 10,
    Copper = 100,
    Desert = 1000
}

impl PodKind {
    pub fn from(ch: char) -> Option<PodKind> {
        match ch {
            'A' => Some(PodKind::Amber),
            'B' => Some(PodKind::Bronze),
            'C' => Some(PodKind::Copper),
            'D' => Some(PodKind::Desert),
            _ => None
        }
    }
    pub fn to_char(self) -> char {
        match self {
            PodKind::Amber => 'A',
            PodKind::Bronze => 'B',
            PodKind::Copper => 'C',
            PodKind::Desert => 'D',
        }
    }
    pub fn goal_col(&self) -> usize {
        match self {
            PodKind::Amber => 3,
            PodKind::Bronze => 5,
            PodKind::Copper => 7,
            PodKind::Desert => 9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pod {
    pub location: Coords,
    pub kind: PodKind,
    pub cur_goals: Vec<Coords>,
    pub num_turns: usize,
}

impl Hash for Pod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.kind.hash(state);
        self.num_turns.hash(state);
    }
}

impl Eq for Pod {}

impl PartialEq for Pod {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.kind == other.kind
    }
}

impl PartialOrd for Pod {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pod {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind == other.kind {
            return self.location.cmp(&other.location);
        }
        self.kind.cmp(&other.kind)
    }
}

impl Pod {
    pub fn new(location: Coords, kind: PodKind, num_turns: usize, use_size_four: bool) -> Pod {
        let mut pod = Pod {
            location,
            kind,
            cur_goals: vec![],
            num_turns
        };
        pod.recalculate_goals(use_size_four);
        pod
    }
    pub fn recalculate_goals(&mut self, use_size_four: bool) {
        let (x, y) = self.location;
        let mut cur_goals = vec![];
        let goal_col = self.kind.goal_col();
        let y_end = if use_size_four { 5 } else { 3 };
        
        if self.num_turns < 2 && !(x == goal_col && y == y_end) {
            if y == 1 {
                if use_size_four {
                    cur_goals.push((goal_col, 4));
                    cur_goals.push((goal_col, 5));    
                }
                cur_goals.push((goal_col, 3));
                cur_goals.push((goal_col, 2));
            } else if self.num_turns == 0 {
                cur_goals.push((1, 1));
                cur_goals.push((2, 1));
                cur_goals.push((4, 1));
                cur_goals.push((6, 1));
                cur_goals.push((8, 1));
                cur_goals.push((10, 1));
                cur_goals.push((11, 1));
                cur_goals.push((goal_col, 2));
                cur_goals.push((goal_col, 3));
                if use_size_four {
                    cur_goals.push((goal_col, 4));
                    cur_goals.push((goal_col, 5));    
                }
            }
        }
        self.cur_goals = cur_goals
            .into_iter()
            .filter(|goal| *goal != self.location)
            .collect()
    }
}